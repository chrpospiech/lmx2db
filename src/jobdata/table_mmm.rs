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

/// Helper function to check whether a parameter of type Option<serde_yaml::Value>
/// is a sequence (array) and checks whether the second element in that sequence is
/// a non-zero float.
/// If the parameter is None, return None.
/// If the parameter is not a sequence or the second element is not a float,
/// return an error.
/// If the second element is a float with value zero, return None.
/// Otherwise, return a Vec<serde_yaml::Value> containing the first two
/// elements of the sequence.
///
/// # Arguments
/// * `param` - An optional serde_yaml::Value to check
///
/// # Returns
/// * Ok(Some(Vec<serde_yaml::Value>)) if the second element is a non-zero float
/// * Ok(None) if the parameter is None or the second element is zero
/// * Err(anyhow::Error) if the parameter is not a sequence or the second element is not a float
fn parse_optional_float_sequence(
    param: &Option<serde_yaml::Value>,
) -> Result<Option<Vec<serde_yaml::Value>>> {
    if let Some(value) = param {
        if let serde_yaml::Value::Sequence(seq) = value {
            if seq.len() >= 2 {
                if let serde_yaml::Value::Number(num) = &seq[1] {
                    if let Some(f) = num.as_f64() {
                        if f != 0.0 {
                            return Ok(Some(seq[..2].to_vec()));
                        } else {
                            return Ok(None);
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "Expected a float as the second element in the sequence"
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Expected a float as the second element in the sequence"
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "Expected at least two elements in the sequence"
                ));
            }
        } else {
            return Err(anyhow::anyhow!("Expected a sequence (array)"));
        }
    }
    Ok(None)
}

/// Extracts values from the `min_max_times` section of the LMX summary file
/// and generates SQL insert statements for the 'mmm' table. The function returns
/// a vector of SQL queries to be executed or written to a file.
/// For each LMX_summary file, at most a single row is inserted into the 'mmm' table.
/// The keys in the `min_max_times` section have to be mapped to a pair of
/// database column names as follows:
/// - "min_comm" -> ("mintask", "mincomm")
/// - "med_comm" -> ("medtask", "medcomm")
/// - "max_comm" -> ("maxtask", "maxcomm")
/// - "min_mpiio" -> ("minmpiiotask", "minmpiio")
/// - "med_mpiio" -> ("medmpiiotask", "medmpiio")
/// - "max_mpiio" -> ("maxmpiiotask", "maxmpiio")
/// - "min_loadimb" -> ("minloadimbtask", "minloadimb")
/// - "med_loadimb" -> ("medloadimbtask", "medloadimb")
/// - "max_loadimb" -> ("maxloadimbtask", "maxloadimb")
/// - "min_io" -> ("miniotask", "minio")
/// - "med_io" -> ("mediotask", "medio")
/// - "max_io" -> ("maxiotask", "maxio")
///
/// For each of these keys, min_max_times[key] is parsed with
/// `parse_optional_float_sequence()`. If the result is Some(Vec),
/// the two values are extracted and added to the insert statement.
/// If the result is None, the corresponding columns are omitted from the insert
/// statement. If the `min_max_times` section is absent or no valid
/// entries are found, no insert statement is generated.
///
/// # Arguments
/// * `lmx_summary` - The parsed LMX summary data
/// * `sqltypes` - The database schema mapping
/// * `args` - Command line arguments controlling processing behavior
///
/// # Returns
/// * A vector of SQL insert statements for the 'mmm' table
pub fn import_into_mmm_table(
    lmx_summary: &LmxSummary,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let mut queries = Vec::new();

    // Check early if 'mmm' table exists in sqltypes to fail fast
    if !sqltypes.contains_key("mmm") {
        return Ok(queries);
    }

    let mmm_section = lmx_summary.get("min_max_times");
    if mmm_section.is_none() {
        if args.verbose || args.dry_run {
            println!("No 'min_max_times' section found in LMX_summary file.");
        }
        return Ok(queries);
    }
    let mmm_section = mmm_section.unwrap();
    let mut columns = vec!["rid".to_string()];
    let mut values = vec![serde_yaml::Value::String("@rid".to_string())];

    let mappings = vec![
        ("min_comm", ("mintask", "mincomm")),
        ("med_comm", ("medtask", "medcomm")),
        ("max_comm", ("maxtask", "maxcomm")),
        ("min_mpiio", ("minmpiiotask", "minmpiio")),
        ("med_mpiio", ("medmpiiotask", "medmpiio")),
        ("max_mpiio", ("maxmpiiotask", "maxmpiio")),
        ("min_loadimb", ("minloadimbtask", "minloadimb")),
        ("med_loadimb", ("medloadimbtask", "medloadimb")),
        ("max_loadimb", ("maxloadimbtask", "maxloadimb")),
        ("min_io", ("miniotask", "minio")),
        ("med_io", ("mediotask", "medio")),
        ("max_io", ("maxiotask", "maxio")),
    ];

    for (key, (col1, col2)) in mappings {
        let param = mmm_section.get(key);
        let parsed = parse_optional_float_sequence(&param.cloned())?;
        if let Some(values_vec) = parsed {
            columns.push(col1.to_string());
            columns.push(col2.to_string());
            values.push(values_vec[0].clone());
            values.push(values_vec[1].clone());
        }
    }

    if values.len() > 1 {
        queries.push("-- Inserting into mmm table;".to_string());
        queries.push(create_import_statement(
            "mmm",
            &columns,
            &[values],
            sqltypes,
        )?);
    }

    Ok(queries)
}
