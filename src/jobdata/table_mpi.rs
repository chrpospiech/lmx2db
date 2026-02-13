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
use crate::jobdata::create_sql::create_import_statement;
use crate::jobdata::mpi_ranks::extract_mpi_rank;
use crate::jobdata::{read_lmx_summary, LmxSummary};
use crate::sqltypes::SqlTypeHashMap;
use anyhow::{bail, Result};

#[cfg(test)]
pub(crate) mod extract_profile;
#[cfg(test)]
pub(crate) mod vec_serde_yaml;

/// Helper function to extract a vector of values from LMX summary type data.
/// This function takes a reference to a single serde_yaml::Value that is
/// expected to be a sequence (array) and returns a vector of serde_yaml::Value.
///
/// # Arguments
/// * `value` - A reference to a serde_yaml::Value that should be a sequence
///
/// Returns a vector of serde_yaml::Value extracted from the sequence.
/// If the input value is not a sequence, an error is returned.
pub fn extract_vector_from_serde_yaml(value: &serde_yaml::Value) -> Result<Vec<serde_yaml::Value>> {
    if let Some(seq) = value.as_sequence() {
        Ok(seq.clone())
    } else {
        bail!(
            "Expected a sequence (array) in LMX summary, but got: {:?}",
            value
        );
    }
}

/// Helper function to extract MPI data from an LMX summary type structure.
/// This function takes a reference to an LMX summary data structure,
/// extracts the MPI rank using the `extract_mpi_rank` function, and then
/// parses the section denoted by the provided `section_key` to extract
/// the relevant data for that MPI rank. It returns these data as
/// Vec<Vec<serde_yaml::Value>>.
///
/// # Arguments
/// * `mpi_profile` - A reference to the LMX summary data structure.
/// * `section_key` - The key in the LMX summary that contains the MPI data to be extracted.
///
/// Returns a vector of vectors of serde_yaml::Value containing the extracted MPI data.
/// If the section key is not found or if the data cannot be properly extracted, an error
/// is returned.
pub fn extract_mpi_data_from_mpi_profile(
    mpi_profile: &LmxSummary,
    section_key: &str,
) -> Result<Vec<Vec<serde_yaml::Value>>> {
    let mpi_rank = extract_mpi_rank(mpi_profile)?;
    let mut result: Vec<Vec<serde_yaml::Value>> = Vec::new();
    let is_detail = section_key.contains("detail");

    if let Some(mpi_profiles) = mpi_profile.get(section_key) {
        if mpi_profiles.is_empty() {
            bail!("No MPI profiles found in section '{}'", section_key);
        }

        for (key, value) in mpi_profiles.iter() {
            let mut row: Vec<serde_yaml::Value> = vec![
                serde_yaml::Value::String("@rid".to_string()),
                serde_yaml::Value::Number(serde_yaml::Number::from(mpi_rank as u64)),
                serde_yaml::Value::String(format!("mpi_call_id('{}')", key)),
            ];

            if !is_detail {
                let vector = extract_vector_from_serde_yaml(value)?;
                if vector.len() != 3 {
                    bail!(
                        "Expected 3 values for MPI profile data in section '{}', but got {}: {:?}",
                        section_key,
                        vector.len(),
                        vector
                    );
                } else {
                    row.extend(vector);
                    result.push(row);
                }
            } else {
                let vector = extract_vector_from_serde_yaml(value)?;
                for elem in vector {
                    let mut detail_row = row.clone();
                    let elem_vector = extract_vector_from_serde_yaml(&elem)?;
                    if elem_vector.len() != 3 {
                        bail!(
                            "Expected 3 values for MPI profile detail data in section '{}', but got {}: {:?}",
                            section_key,
                            elem_vector.len(),
                            elem_vector
                        );
                    } else {
                        detail_row.extend(elem_vector);
                        result.push(detail_row);
                    }
                }
            }
        }
    } else {
        bail!("'{}' key not found in LMX summary", section_key);
    }

    Ok(result)
}

/// Extracts MPI profile data from YAML files found by `find_lmx_type_files`
/// and processes it to generate SQL queries for database insertion.
/// This function reads the MPI profile files, extracts relevant data using
/// `extract_mpi_data_from_mpi_profile`, and then generates SQL insert statements
/// based on the provided `sqltypes` schema mapping. The generated SQL queries
/// are returned as a vector of strings.
/// The MPI profile files are expected to be in YAML format and can be parsed
/// into the LMX summary data structure. The function handles both regular and
/// detail sections of the MPI profile data, generating appropriate SQL queries
/// for each case.
///
/// # Arguments
/// * `file_name` - The reference LMX summary file name to find MPI profile files.
/// * `sqltypes` - A HashMap containing the database schema mapping for generating SQL queries.
/// * `args` - Command line arguments including verbosity and dry-run
///
/// # Returns
/// A Result containing a vector of SQL insert statements for the MPI profile data or an error.
/// Errors if the MPI profile files cannot be found, read, parsed,
/// or if the data cannot be extracted properly.
pub fn import_into_mpi_table(
    file_name: &str,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let mut query_list: Vec<String> = Vec::new();
    let mpi_profile_files = find_lmx_type_files(file_name, "MPI")?;
    if mpi_profile_files.is_empty() {
        if args.verbose || args.dry_run {
            println!("No MPI profile files found for file '{}'", file_name);
        }
        return Ok(query_list);
    }
    for mpi_profile_file in mpi_profile_files {
        let mpi_profile = read_lmx_summary(&mpi_profile_file)?;
        let mpi_data = extract_mpi_data_from_mpi_profile(&mpi_profile, "MPI_rank_summary")?;
        if !mpi_data.is_empty() {
            query_list.push(format!(
                "-- Inserting MPI profile data from file {};",
                mpi_profile_file
            ));
            query_list.push(create_import_statement(
                "mpi",
                &[
                    "rid".to_string(),
                    "tid".to_string(),
                    "mid".to_string(),
                    "calls".to_string(),
                    "avgbytes".to_string(),
                    "time".to_string(),
                ],
                &mpi_data,
                sqltypes,
            )?);
        }

        let mpi_detail_data = extract_mpi_data_from_mpi_profile(&mpi_profile, "MPI_rank_details")?;
        if !mpi_detail_data.is_empty() {
            query_list.push(format!(
                "-- Inserting MPI profile detail data from file {};",
                mpi_profile_file
            ));
            query_list.push(create_import_statement(
                "mpi_details",
                &[
                    "rid".to_string(),
                    "tid".to_string(),
                    "mid".to_string(),
                    "calls".to_string(),
                    "avgbytes".to_string(),
                    "time".to_string(),
                ],
                &mpi_detail_data,
                sqltypes,
            )?);
        }
    }
    Ok(query_list)
}
