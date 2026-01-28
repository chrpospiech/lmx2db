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

/// Extracts the environment variables from the environ section of the LMX summary file
/// and generates SQL insert statements for the 'environ' table. The function returns
/// a vector of SQL queries to be executed or written to a file.
///
/// # Arguments
/// * `lmx_summary` - Contents of the LMX summary file as a HashMap
/// * `sqltypes` - HashMap containing the database schema mapping for generating SQL queries
/// * `args` - Command line arguments controlling processing behavior
///
/// # Returns
/// A vector of SQL insert statements for the 'environ' table. Returns silently with an
/// empty vector if the 'environ' table is not present in `sqltypes` or if the LMX summary
/// does not contain an 'environ' section.
///
/// # Errors
/// - Returns an error if a value in a sequence for a given key is not a string.
/// - Returns an error if a value for a key is neither a string nor a sequence (unexpected type).
/// - Propagates any error returned by `create_import_statement` when generating the SQL.
///
pub fn import_into_environ_table(
    lmx_summary: &LmxSummary,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    let key_list: Vec<String> = vec!["rid".to_string(), "k".to_string(), "value".to_string()];
    let mut value_list: Vec<Vec<serde_yaml::Value>> = Vec::new();
    let mut query_list: Vec<String> = Vec::new();

    // Check early if 'environ' table exists in sqltypes to fail fast
    if !sqltypes.contains_key("environ") {
        return Ok(query_list);
    }

    let env_section = lmx_summary.get("environ");
    if env_section.is_none() {
        if args.verbose || args.dry_run {
            println!("No 'environ' section found in LMX_summary file.");
        }
        return Ok(query_list);
    }
    let env_section = env_section.unwrap();
    for (key, v) in env_section.iter() {
        match v {
            serde_yaml::Value::Sequence(seq) => {
                let str_vec: Result<Vec<String>, _> = seq
                    .iter()
                    .map(|val| {
                        val.as_str().map(String::from).ok_or_else(|| {
                            anyhow::anyhow!("Value in sequence for key '{}' is not a string", key)
                        })
                    })
                    .collect();

                let str_vec = str_vec?;
                value_list.push(vec![
                    serde_yaml::Value::String("@rid".to_string()),
                    serde_yaml::Value::String(key.to_string()),
                    serde_yaml::Value::String(str_vec.join("")),
                ]);
            }
            serde_yaml::Value::String(s) => {
                value_list.push(vec![
                    serde_yaml::Value::String("@rid".to_string()),
                    serde_yaml::Value::String(key.to_string()),
                    serde_yaml::Value::String(s.clone()),
                ]);
            }
            _ => {
                anyhow::bail!(
                    "Value for key '{}' is {:?}, expected String or Sequence",
                    key,
                    v
                )
            }
        }
    }

    if !value_list.is_empty() {
        query_list.push("-- Inserting into environ table;".to_string());
        query_list.push(create_import_statement(
            "environ",
            &key_list,
            &value_list,
            sqltypes,
        )?);
    }

    Ok(query_list)
}

#[cfg(test)]
pub(crate) mod test_empty_queries;
#[cfg(test)]
pub(crate) mod test_errors;
#[cfg(test)]
pub(crate) mod test_non_empty_queries;
