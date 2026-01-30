// Copyright 2026 lmx2db C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cmdline::CliArgs;
use crate::jobdata::create_sql::create_import_statement;
use crate::jobdata::LmxSummary;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

/// Helper function to check whether a parameter `value` of type
/// Option<serde_yaml::Value> is a sequence (array) of floats.
/// Two additional &str parameters `rank` and `section`
/// are only used to create meaningful error messages.
/// If the parameter is None, bail out with message
/// "Missing section {section} for rank {rank}".
/// If the parameter is not a sequence or any element is not a float,
/// return an error with appropriate message including rank and section.
/// Otherwise, return a Vec<serde_yaml::Value> containing the elements of the sequence.
///
/// # Arguments
/// * `value` - An optional &serde_yaml::Value to check
/// * `rank` - A &str indicating the rank for error messages
/// * `section` - A &str indicating the section for error messages
///
/// # Returns
/// * `Vec<serde_yaml::Value>` if the parameter is a sequence of floats
/// * `Err` if the parameter is None or not a sequence or contains non-float elements
///
fn parse_optional_float_array(
    value: &Option<&serde_yaml::Value>,
    rank: &str,
    section: &str,
) -> Result<Vec<serde_yaml::Value>> {
    if let Some(value) = value {
        if let serde_yaml::Value::Sequence(seq) = value {
            for elem in seq {
                if let serde_yaml::Value::Number(num) = elem {
                    if num.as_f64().is_none() {
                        return Err(anyhow::anyhow!(
                            "Expected a float in {} for rank {}, but got non-float number value: {:?}",
                            section,
                            rank,
                            elem
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Expected a float in {} for rank {}, but got value: {:?}",
                        section,
                        rank,
                        elem
                    ));
                }
            }
            return Ok(seq.clone());
        } else {
            return Err(anyhow::anyhow!(
                "Expected a sequence (array) for {} in rank {}, but got value: {:?}",
                section,
                rank,
                value
            ));
        }
    }
    Err(anyhow::anyhow!(
        "Missing section {} for rank {}",
        section,
        rank
    ))
}

/// Helper function to check whether a parameter of type Option<serde_yaml::Value>
/// is a sequence (array) of strings.
/// If the parameter is None, return None.
/// If the parameter is not a sequence or any element is not a string,
/// return an error.
/// Otherwise, return a Vec<serde_yaml::Value> with two elements:
/// - the first element of the sequence
/// - the remaining elements joined to a single string without separator.
///
/// # Arguments
/// * `value` - An optional &serde_yaml::Value to check
/// * `rank` - A &str indicating the rank for error messages
/// * `section` - A &str indicating the section for error messages
///
/// # Returns
/// * `Vec<serde_yaml::Value>` if the parameter is a sequence of strings
/// * `Err` if the parameter is None or not a sequence or contains non-string elements
///
fn parse_optional_string_array(
    value: &Option<&serde_yaml::Value>,
    rank: &str,
    section: &str,
) -> Result<Vec<serde_yaml::Value>> {
    if let Some(value) = value {
        if let serde_yaml::Value::Sequence(seq) = value {
            let mut string_elems = Vec::new();
            for elem in seq {
                if let serde_yaml::Value::String(s) = elem {
                    string_elems.push(s.clone());
                } else {
                    return Err(anyhow::anyhow!(
                        "Expected a string in {} for rank {}, but got value: {:?}",
                        section,
                        rank,
                        elem
                    ));
                }
            }
            if !string_elems.is_empty() {
                let first_elem = serde_yaml::Value::String(string_elems[0].clone());
                let remaining_joined = serde_yaml::Value::String(string_elems[1..].join(""));
                return Ok(vec![first_elem, remaining_joined]);
            } else {
                return Err(anyhow::anyhow!(
                    "Expected at least one element for {} in rank {}, but got empty sequence",
                    section,
                    rank
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "Expected a sequence (array) for {} in rank {}, but got value: {:?}",
                section,
                rank,
                value
            ));
        }
    }
    Err(anyhow::anyhow!(
        "Missing section {} for rank {}",
        section,
        rank
    ))
}

/// Extracts values from several sections of the LMX_summary file
/// and generates SQL queries to insert them into table tasks in the database.
/// Different sections are of the LMX summary file are processed to fill
/// different columns of the tasks table. The keys for these sections are
/// integers [0, ..., n-1], where n is the number of tasks (i.e. MPI ranks).
/// For each task, a single row is inserted into the tasks table.
///
/// The keys in in every section go to the column `tid` in the tasks table.
/// The values in section `affinity` are to be processed by the helper function
/// `parse_optional_string_array()`. This function returns a `Vec<serde_yaml::Value>`
/// containing two string elements: the first element of the sequence is processed by a
/// stored function to provide the value for column `lid`. The second element of the
/// sequence is used to provide the value for column `affinity`.
///
/// The values in section `rank_summary` are to be processed by the helper function
/// `parse_optional_float_array()`. This function returns a Vec<> to be inserted into
/// columns `elapsed`, `usertime`, `systime`, `memory`, `vmemory` in this order.
///
/// The values in section `communication_times` - if present - are to be processed
/// by the helper function `parse_optional_float_sequence()`. This function returns
/// a Vec<serde_yaml::Value>. The first element and third element of this sequence
/// is to be inserted into columns `comm` and `mpiio`, respectively.
///
/// The values in section `load_imbalance_times` - if present - are to be processed
/// by the helper function `parse_optional_float_sequence()`. This function returns
/// a Vec<serde_yaml::Value>. Only the first element of this sequence
/// is to be inserted into column `loadimb`.
///
/// The function returns a Vec<String> containing 3 SQL statements.
/// The first statement is a comment "-- Inserting into tasks table;".to_string(),
/// the second statement is a single call to a stored function, and
/// the last statement inserts all task records into the tasks table.
///
/// # Arguments
/// * `lmx_summary` - Reference to the LMX summary data structure
/// * `sqltypes` - Reference to the SQL type mapping for generating SQL queries
/// * `args` - Reference to the command line arguments controlling processing behavior
///
/// # Returns
/// * `Result<Vec<serde_yaml::Value>>`
/// - Ok containing an Option<Vec<serde_yaml::Value>> if all checks pass
/// - Ok(None) if the parameter is None or a section is not present
/// - Err otherwise
///
pub fn import_into_tasks_table(
    lmx_summary: &LmxSummary,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let mut queries = Vec::new();

    // Check early if 'tasks' table exists in sqltypes to fail fast
    if !sqltypes.contains_key("tasks") {
        return Ok(queries);
    }

    // We check for the existence of section 'affinity' and fail
    // if it is missing, as this section is mandatory.
    if !lmx_summary.contains_key("affinity") {
        return Err(anyhow::anyhow!(
            "Missing mandatory section 'affinity' in LMX summary file"
        ));
    }
    let aff_section = &lmx_summary["affinity"];

    // Start building the vector of keys.
    let mut keys: Vec<String> = vec![
        "rid".to_string(),
        "tid".to_string(),
        "lid".to_string(),
        "affinity".to_string(),
    ];

    // We check for the existence of section 'rank_summary' and fail
    // if it is missing, as this section is mandatory.
    if !lmx_summary.contains_key("rank_summary") {
        return Err(anyhow::anyhow!(
            "Missing mandatory section 'rank_summary' in LMX summary file"
        ));
    }
    let rank_sum_section = &lmx_summary["rank_summary"];
    keys.extend(vec![
        "elapsed".to_string(),
        "usertime".to_string(),
        "systime".to_string(),
        "memory".to_string(),
        "vmemory".to_string(),
    ]);

    // Optional sections
    let comm_times_section = lmx_summary.get("communication_times");
    if comm_times_section.is_some() {
        keys.push("comm".to_string());
        keys.push("mpiio".to_string());
    } else if args.verbose || args.dry_run {
        println!("No 'communication_times' section found in LMX_summary file.");
    }
    let loadimb_times_section = lmx_summary.get("load_imbalance_times");
    if loadimb_times_section.is_some() {
        keys.push("loadimb".to_string());
    } else if args.verbose || args.dry_run {
        println!("No 'load_imbalance_times' section found in LMX_summary file.");
    }

    // Now process each task (i.e. each key in aff_section)
    let num_tasks = aff_section.len();
    let mut value_vector: Vec<serde_yaml::Value> = Vec::new();
    for i in 0..num_tasks {
        let rank_str = i.to_string();
        // Extract affinity values
        let aff_values =
            parse_optional_string_array(&aff_section.get(&rank_str), &rank_str, "affinity")?;
        // Start building the values for this task
        let mut values: Vec<serde_yaml::Value> = vec![
            serde_yaml::Value::String("@rid".to_string()),
            serde_yaml::Value::String(rank_str.clone()),
            // lid is processed by stored function location_id()
            serde_yaml::Value::String(format!(
                "location_id({}, @cl_name, 'nodes')",
                aff_values[0].as_str().unwrap()
            )),
            // affinity
            aff_values[1].clone(),
        ];
        // Extract rank_summary values
        let rank_sum_values = parse_optional_float_array(
            &rank_sum_section.get(&rank_str),
            &rank_str,
            "rank_summary",
        )?;
        // Append rank_summary values
        values.extend(rank_sum_values);
        // Extract communication_times values if present
        if let Some(comm_section) = comm_times_section {
            let comm_values = parse_optional_float_array(
                &comm_section.get(&rank_str),
                &rank_str,
                "communication_times",
            )?;
            if comm_values.len() < 3 {
                return Err(anyhow::anyhow!(
                    "Expected at least 3 communication_times entries for rank {}, but got {}",
                    rank_str,
                    comm_values.len()
                ));
            }
            // Append comm (first element) and mpiio (third element)
            values.push(comm_values[0].clone());
            values.push(comm_values[2].clone());
        }
        // Extract load_imbalance_times values if present
        if let Some(loadimb_section) = loadimb_times_section {
            let loadimb_values = parse_optional_float_array(
                &loadimb_section.get(&rank_str),
                &rank_str,
                "load_imbalance_times",
            )?;
            // Append loadimb (first element)
            values.push(loadimb_values[0].clone());
        }
        value_vector.push(serde_yaml::Value::Sequence(values));
    }

    queries.push("-- Inserting into tasks table;".to_string());
    queries.push("SET @cl_name = cluster_name(@clid);".to_string());
    queries.push(create_import_statement(
        "tasks",
        &keys,
        &[value_vector],
        sqltypes,
    )?);

    Ok(queries)
}
