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
use crate::jobdata::table_runs::find_file::find_and_read_settings_file;
use anyhow::Result;
use std::collections::HashMap;

#[cfg(test)]
pub(crate) mod test_misc_columns;
#[cfg(test)]
pub(crate) mod test_settings_columns;

/// Determines the values for miscellaneous columns `has_MPItrace`
/// and `has_iprof` in the 'runs' table based on the presence
/// of corresponding LMX type files in the same directory as the
/// provided LMX summary file.
///
/// Arguments:
/// * `file_name` - The LMX summary file name.
///
/// Returns:
/// Result<Vec<(String, serde_yaml::Value)>> - A vector of tuples of column name and value
/// indicating presence (1) or absence (0) of the type file.
///
/// Errors if there are issues determining the directory or globbing files.
pub fn determine_misc_columns(file_name: &str) -> Result<Vec<(String, serde_yaml::Value)>> {
    // Use the find_lmx_type_files function to check for the presence of type files
    let mpi_type_files = find_lmx_type_files(file_name, "MPI")?;
    let itimer_type_files = find_lmx_type_files(file_name, "itimer")?;
    let result = vec![
        (
            "has_MPItrace".to_string(),
            serde_yaml::Value::Number(if mpi_type_files.is_empty() {
                serde_yaml::Number::from(0)
            } else {
                serde_yaml::Number::from(1)
            }),
        ),
        (
            "has_iprof".to_string(),
            serde_yaml::Value::Number(if itimer_type_files.is_empty() {
                serde_yaml::Number::from(0)
            } else {
                serde_yaml::Number::from(1)
            }),
        ),
    ];
    Ok(result)
}

/// Updates columns in the runs table based on settings provided by
/// the user in the settings file associated with the given LMX summary file.
///
/// Arguments:
/// * `file_name` - The LMX summary file name.
/// * `runs_columns` - the runs part of the sqltypes HashMap.
/// * `args` - Command line arguments.
///
/// Returns:
/// Vec<(String, serde_yaml::Value)> - A vector of tuples of column names and values
/// extracted from the settings file.
///
/// Errors if there are issues reading or parsing the settings file.
pub fn determine_settings_columns(
    file_name: &str,
    runs_columns: &HashMap<String, String>,
    args: &CliArgs,
) -> Vec<(String, serde_yaml::Value)> {
    if args.verbose || args.dry_run {
        println!(
            "Updating columns in table runs with data from settings file: {}",
            args.settings_file
        );
    }
    let mut result: Vec<(String, serde_yaml::Value)> = Vec::new();
    let settings_map = match find_and_read_settings_file(file_name, args, true) {
        Ok(map) => map,
        Err(e) => {
            println!("Ignoring: {}", e);
            return result;
        }
    };
    for (key, value) in settings_map.iter() {
        if runs_columns.contains_key(key as &str) {
            result.push((key.clone(), value.clone()));
        }
    }
    result
}
