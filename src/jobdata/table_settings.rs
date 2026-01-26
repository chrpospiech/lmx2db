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
use crate::jobdata::table_runs::find_file::find_and_read_settings_file;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

/// Checks whether there is a file args.settings_file in the same directory
/// as the LMX summary file, and if so, reads additional settings from it.
/// These settings are then used to generate SQL insert statements for the
/// 'settings' table. The function returns a vector of SQL queries to be executed
/// or written to a file.
///
/// The file args.settings_file is expected to be in YAML format and contain
/// key-value pairs of type String-String representing additional settings.
/// The key-value pairs with keys matching a column name in the `runs` table
/// are ignored, as they are handled separately in function `import_into_runs_table`.
///
/// # Arguments
/// * `file_name` - Path to the LMX summary file being processed
/// * `sqltypes` - HashMap containing the database schema mapping for generating SQL queries
/// * `args` - Command line arguments controlling processing behavior
///
/// # Returns
/// A vector of SQL insert statements for the 'settings' table
///
/// # Errors
/// - Returns silent without error if no settings file is found
/// - Returns an error if the settings file cannot be read or parsed
pub fn import_into_settings_table(
    file_name: &str,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let key_list: Vec<String> = vec!["rid".to_string(), "k".to_string(), "value".to_string()];
    let mut value_list: Vec<Vec<serde_yaml::Value>> = Vec::new();
    let mut query_list: Vec<String> = Vec::new();
    
    let settings_yaml = match find_and_read_settings_file(file_name, args) {
        Ok(map) => map,
        Err(e) => {
            if args.verbose || args.dry_run {
                println!("Ignoring: {}", e);
            }
            return Ok(query_list);
        }
    };

    for (key, value) in settings_yaml.iter() {
        // Skip keys that are part of the 'runs' table
        if sqltypes.contains_key("runs") && sqltypes["runs"].contains_key(key) {
            continue;
        }
        value_list.push(vec![
            serde_yaml::Value::String("@rid".to_string()),
            serde_yaml::Value::String(key.clone()),
            value.clone(),
        ]);
    }
    if !value_list.is_empty() {
        query_list.push(create_import_statement(
            "settings",
            &key_list,
            &value_list,
            sqltypes,
        )?);
    }
    Ok(query_list)
}

#[cfg(test)]
pub(crate) mod test_import_settings;
