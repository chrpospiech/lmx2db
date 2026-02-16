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
use crate::globbing::find_lmx_type_files;
use crate::jobdata::base_data::extract_base_data_key;
use crate::jobdata::create_sql::create_import_statement;
use crate::jobdata::{read_lmx_summary, LmxSummary};
use crate::sqltypes::SqlTypeHashMap;
use anyhow::{anyhow, bail, Result};

#[cfg(test)]
pub(crate) mod extract_full_function_name;
#[cfg(test)]
pub(crate) mod extract_full_library_name;
#[cfg(test)]
pub(crate) mod extract_full_name;
#[cfg(test)]
pub(crate) mod extract_iprof_ticks;
#[cfg(test)]
pub(crate) mod import_into_iprof;

/// Helper function to extract the number of interval timer profiler ticks.
///
/// # Arguments
/// * `value` - A reference to a serde_yaml::Value that should be a sequence
///
/// # Returns
/// * `Result<i32>` - The number of interval timer profiler ticks as an i32.
///
/// # Errors
/// * Returns an error if the input value is not a sequence.
/// * Returns an error if the first value is not an integer.
/// * Returns an error if the integer value is out of i32 range.
pub fn extract_iprof_ticks(value: &serde_yaml::Value) -> Result<i32> {
    let seq = value.as_sequence().ok_or_else(|| {
        anyhow!(
            "Expected a sequence with an integer for interval timer profiler ticks, but got: {:?}",
            value
        )
    })?;
    let first_value = seq.first().ok_or_else(|| {
        anyhow!(
            "Expected a sequence with an integer for interval timer profiler ticks, but got: {:?}",
            value
        )
    })?;
    let ticks = first_value.as_i64().ok_or_else(|| {
        anyhow!(
            "Expected a sequence with an integer for interval timer profiler ticks, but got: {:?}",
            value
        )
    })?;
    if ticks < i32::MIN as i64 || ticks > i32::MAX as i64 {
        bail!(
            "Interval timer profiler ticks value {} is out of i32 range ({}..={})",
            ticks,
            i32::MIN,
            i32::MAX
        );
    }
    Ok(ticks as i32)
}

/// Helper function to extract a full library or function name.
///
/// # Arguments
/// * `value` - A reference to a serde_yaml::Value that should be a sequence
///
/// # Returns
/// * `Result<String>` - The full library or function name as a joined string.
///
/// # Errors
/// * Returns an error if the input value is not a sequence.
/// * Returns an error if any member is not a string.
/// * Returns an error if the sequence is empty.
pub fn extract_full_name(value: &serde_yaml::Value) -> Result<String> {
    let seq = value.as_sequence().ok_or_else(|| {
        anyhow!(
            "Expected a sequence for full name extraction, but got: {:?}",
            value
        )
    })?;
    let mut full_name_parts = Vec::new();
    for member in seq {
        let part = member.as_str().ok_or_else(|| {
            anyhow!(
                "Expected all members of the sequence to be strings, but got: {:?}",
                member
            )
        })?;
        full_name_parts.push(part);
    }
    let full_name = full_name_parts.join("");
    if full_name.is_empty() {
        bail!("Expected a non-empty sequence for full name extraction, but got an empty sequence");
    }
    Ok(full_name)
}

/// Helper function to extract a full library name from LmxSummary type data
/// and a given short name.
///
/// # Arguments
/// * `iprof` - A reference to a LmxSummary struct containing the profiling data.
/// * `short_name` - A reference to a String representing the short name of the library.
///
/// # Returns
/// * `Result<String>` - The full library name as a string.
///
/// # Errors
/// * Returns an error if the given short name is not found in the LmxSummary data.
/// * Returns an error if the full name cannot be extracted from the found library data.
pub fn extract_full_library_name(iprof: &LmxSummary, short_name: &String) -> Result<String> {
    let name_table = iprof
        .get("library_names")
        .ok_or_else(|| anyhow!("'library_names' section not found in LmxSummary data"))?;
    let value = name_table.get(short_name).ok_or_else(|| {
        anyhow!(
            "Short name '{}' not found in 'library_names' section of LmxSummary data",
            short_name
        )
    })?;
    let full_name = extract_full_name(value)?;
    Ok(full_name)
}

/// Helper function to extract a full function name from LmxSummary type data
/// for a given lib_name and a given short name.
///
/// # Arguments
/// * `iprof` - A reference to a LmxSummary struct containing the profiling data.
/// * `lib_name` - A reference to a String representing the short name of the library.
/// * `short_name` - A reference to a String representing the short name of the function.
///
/// # Returns
/// * `Result<String>` - The full function name as a string.
///
/// # Errors
/// * Returns an error if the given lib_name is not found in the LmxSummary data.
/// * Returns an error if the given short name is not found in the found library data.
/// * Returns an error if the full name cannot be extracted from the found function data.
pub fn extract_full_function_name(
    iprof: &LmxSummary,
    lib_name: &String,
    short_name: &String,
) -> Result<String> {
    let lib_table = iprof
        .get("subroutine_names")
        .ok_or_else(|| anyhow!("'subroutine_names' section not found in LmxSummary data"))?;
    let lib_value = lib_table.get(lib_name).ok_or_else(|| {
        anyhow!(
            "Library name '{}' not found in 'subroutine_names' section of LmxSummary data",
            lib_name
        )
    })?;
    let func_table = lib_value.as_mapping().ok_or_else(|| {
        anyhow!(
            "Expected a mapping for library '{}' in 'subroutine_names' section, but got: {:?}",
            lib_name,
            lib_value
        )
    })?;
    let func_value = func_table.get(serde_yaml::Value::String(short_name.to_string()))
        .ok_or_else(|| anyhow!("Short name '{}' not found in 'subroutine_names' section for library '{}' in LmxSummary data", short_name, lib_name))?;
    let full_name = extract_full_name(func_value)?;
    Ok(full_name)
}

/// Extracts interval timer profile data from YAML files found by `find_lmx_type_files`
/// and processes it to generate SQL queries for database insertion.
/// The interval timer profile files are expected to be in YAML format and can be parsed
/// into the LMX summary data structure. This function reads this data structure,
/// extracts relevant data using `extract_iprof_ticks`, and then generates SQL insert
/// statements based on the provided `sqltypes` schema mapping. The generated SQL queries
/// are returned as a vector of strings.
/// The function returns without error and an empty `Vec<String>` if no interval timer
/// profile data is found by `find_lmx_type_files`.
///
/// # Arguments
/// * `file_name` - The reference LMX summary file name to find interval timer profile files.
/// * `sqltypes` - A HashMap containing the database schema mapping for generating SQL queries.
/// * `args` - Command line arguments including verbosity and dry-run
///
/// # Returns
/// `Result<Vec<String>>` - A Result containing a vector of SQL insert statements.
///
/// # Errors
/// * Returns an error if any of the found interval timer profile files cannot be read or parsed.
/// * Returns an error if the relevant interval timer profile data cannot be extracted properly.
pub fn import_into_iprof_table(
    file_name: &str,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let mut query_list: Vec<String> = Vec::new();
    let iprof_files = find_lmx_type_files(file_name, "itimer")?;
    if iprof_files.is_empty() {
        // No interval timer profile files found, return empty query list without error
        return Ok(query_list);
    }
    for iprof_file in iprof_files {
        let iprof_data = read_lmx_summary(&iprof_file)?;
        let my_mpi_rank = extract_base_data_key(&iprof_data, "my_MPI_rank")?;
        let total_ticks = extract_base_data_key(&iprof_data, "itimer_ticks_total")?;
        // Return early without error if the total_ticks value is zero, as there would be no
        // meaningful data to insert
        if total_ticks == 0 {
            if args.verbose || args.dry_run {
                println!(
                    "Skipping file '{}' for MPI rank {} because total_ticks is zero",
                    iprof_file, my_mpi_rank
                );
            }
            continue;
        }
        // Now we can create a first SQL import statement for the iprof table with the total ticks value.
        let table_name = "iprof";
        let total = "__total__";
        let keys = &[
            "rid".to_string(),
            "tid".to_string(),
            "routine_id".to_string(),
            "ticks".to_string(),
        ];
        let values = &[vec![
            serde_yaml::Value::String("@rid".to_string()),
            serde_yaml::Value::Number(my_mpi_rank.into()),
            serde_yaml::Value::String(format!("routine_id('{}','{}')", total, total)),
            serde_yaml::Value::Number(total_ticks.into()),
        ]];
        let sql_query = create_import_statement(table_name, keys, values, sqltypes)?;
        query_list.push(sql_query);
        // We check whether iprof_data contains a section "library_histogram" with the expected
        // structure of a non-empty HashMap<String, serde_yaml::Value>, and if so, we
        // loop through its keys and values to create a second SQL import statement
        // for the iprof table with the ticks value for each library.
        let histogram = iprof_data.get("library_histogram");
        if histogram.is_none() {
            if args.verbose || args.dry_run {
                println!(
                    "Skipping 'library_histogram' section in file '{}' for MPI rank {} because it was not found",
                    iprof_file, my_mpi_rank
                );
            }
            continue;
        }
        let histogram = histogram.unwrap();
        if histogram.is_empty() {
            if args.verbose || args.dry_run {
                println!(
                    "Skipping 'library_histogram' section in file '{}' for MPI rank {} because it is empty",
                    iprof_file, my_mpi_rank
                );
            }
            continue;
        }
        // Process library_histogram data
        let mut value_list: Vec<Vec<serde_yaml::Value>> = Vec::new();
        for (lib_short_name, lib_data) in histogram {
            let lib_full_name = extract_full_library_name(&iprof_data, lib_short_name)?;
            // Escape single quotes in names before embedding into SQL string literals
            let lib_full_name_escaped = lib_full_name.replace('\'', "''");
            let total_escaped = total.replace('\'', "''");
            let lib_ticks = extract_iprof_ticks(lib_data)?;
            value_list.push(vec![
                serde_yaml::Value::String("@rid".to_string()),
                serde_yaml::Value::Number(my_mpi_rank.into()),
                serde_yaml::Value::String(format!("routine_id('{}','{}')", lib_full_name_escaped, total_escaped)),
                serde_yaml::Value::Number(lib_ticks.into()),
            ]);
        }
        let sql_query = create_import_statement(table_name, keys, &value_list, sqltypes)?;
        query_list.push(sql_query);
        // We check whether iprof_data contains a section "flat_profile" with the expected
        // structure of a non-empty HashMap<String, HashMap<String, serde_yaml::Value>>, and if so, we
        // loop through its keys and values to create further SQL import statements for the iprof table
        // with the ticks value for each function.
        let flat_profile = iprof_data.get("flat_profile");
        if flat_profile.is_none() {
            if args.verbose || args.dry_run {
                println!(
                    "Skipping 'flat_profile' section in file '{}' for MPI rank {} because it was not found",
                    iprof_file, my_mpi_rank
                );
            }
            continue;
        };
        let flat_profile = flat_profile.unwrap();
        if flat_profile.is_empty() {
            if args.verbose || args.dry_run {
                println!(
                    "Skipping 'flat_profile' section in file '{}' for MPI rank {} because it is empty",
                    iprof_file, my_mpi_rank
                );
            }
            continue;
        }
        // Process flat_profile data
        let mut value_list: Vec<Vec<serde_yaml::Value>> = Vec::new();
        for (lib_short_name, func_table) in flat_profile {
            let lib_full_name = extract_full_library_name(&iprof_data, lib_short_name)?;
            let func_table_map = match func_table.as_mapping() {
                Some(mapping) => mapping,
                None => {
                    if args.verbose || args.dry_run {
                        println!(
                            "Skipping library '{}' in 'flat_profile' section because it is not a mapping, but got: {:?}",
                            lib_short_name,
                            func_table
                        );
                    }
                    continue;
                }
            };
            for (func_short_name_value, func_data) in func_table_map {
                let func_short_name = func_short_name_value.as_str()
                    .ok_or_else(|| anyhow!(
                        "Expected a string for function short name in 'flat_profile' section, but got: {:?}",
                        func_short_name_value
                    ))?;
                let func_full_name = extract_full_function_name(
                    &iprof_data,
                    lib_short_name,
                    &func_short_name.to_string(),
                )?;
                let func_ticks = extract_iprof_ticks(func_data)?;
                value_list.push(vec![
                    serde_yaml::Value::String("@rid".to_string()),
                    serde_yaml::Value::Number(my_mpi_rank.into()),
                    serde_yaml::Value::String(format!(
                        "routine_id('{}','{}')",
                        lib_full_name.replace('\'', "''"),
                        func_full_name.replace('\'', "''"),
                    )),
                    serde_yaml::Value::Number(func_ticks.into()),
                ]);
            }
        }
        let sql_query = create_import_statement(table_name, keys, &value_list, sqltypes)?;
        query_list.push(sql_query);
    }

    Ok(query_list)
}
