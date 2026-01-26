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

use crate::jobdata::checktypes::check_type;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;
use regex::Regex;

#[cfg(test)]
pub(crate) mod test_import;
#[cfg(test)]
pub(crate) mod test_update;

pub fn create_import_statement(
    table_name: &str,
    keys: &[String],
    values: &[Vec<serde_yaml::Value>],
    sqltypes: &SqlTypeHashMap,
) -> Result<String> {
    // First, check types
    check_type(table_name, keys, values, sqltypes)?;

    // The following regex will be used multiple times
    let id_pattern = Regex::new(r"@\w+id").unwrap();

    let value_rows: Vec<String> = values
        .iter()
        .map(|value_row| {
            let row_values: Vec<String> = value_row
                .iter()
                .map(|v| match v {
                    serde_yaml::Value::String(s) => {
                        if id_pattern.is_match(s) {
                            s.clone()
                        } else {
                            format!("'{}'", s.replace("'", "''"))
                        }
                    }
                    serde_yaml::Value::Number(n) => n.to_string(),
                    serde_yaml::Value::Bool(b) => {
                        if *b {
                            "1".to_string()
                        } else {
                            "0".to_string()
                        }
                    }
                    serde_yaml::Value::Null => "NULL".to_string(),
                    _ => "'[UNSUPPORTED TYPE]'".to_string(),
                })
                .collect();
            format!("({})", row_values.join(", "))
        })
        .collect();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES\n{};",
        table_name,
        keys.join(", "),
        value_rows.join(",\n")
    );
    Ok(sql)
}

pub fn create_update_statement(
    table_name: &str,
    column: &[(String, serde_yaml::Value)],
    where_clause: &str,
    sqltypes: &SqlTypeHashMap,
) -> Result<String> {
    // First, check types - convert to new API format
    let keys: Vec<String> = column.iter().map(|(k, _)| k.clone()).collect();
    let values: Vec<Vec<serde_yaml::Value>> = vec![column.iter().map(|(_, v)| v.clone()).collect()];
    check_type(table_name, &keys, &values, sqltypes)?;

    // The following regex will be used multiple times
    let id_pattern = Regex::new(r"@\w+id").unwrap();

    let set_clauses: Vec<String> = column
        .iter()
        .map(|(k, v)| {
            let value_str = match v {
                serde_yaml::Value::String(s) => {
                    if id_pattern.is_match(s) {
                        s.clone()
                    } else {
                        format!("'{}'", s.replace("'", "''"))
                    }
                }
                serde_yaml::Value::Number(n) => n.to_string(),
                serde_yaml::Value::Bool(b) => {
                    if *b {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                }
                serde_yaml::Value::Null => "NULL".to_string(),
                _ => "'[UNSUPPORTED TYPE]'".to_string(),
            };
            format!("{} = {}", k, value_str)
        })
        .collect();

    let sql = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name,
        set_clauses.join(", "),
        where_clause
    );
    Ok(sql)
}
