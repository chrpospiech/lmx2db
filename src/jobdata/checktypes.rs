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

use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;
use regex::Regex;

#[cfg(test)]
pub(crate) mod elementary;
#[cfg(test)]
pub(crate) mod wrong_values;

pub fn check_type(
    table_name: &str,
    column: &[(String, serde_yaml::Value)],
    sqltypes: &SqlTypeHashMap,
) -> Result<()> {
    let table_map = sqltypes.get(table_name);
    if table_map.is_none() {
        anyhow::bail!("Table {} not found in type check map", table_name);
    }
    let table_map = table_map.unwrap();
    // The following regexes will be used multiple times
    let id_pattern = Regex::new(r"@\w+id").unwrap();
    let varbinary_pattern = Regex::new(r"varbinary\((\d+)\)").unwrap();
    let varchar_pattern = Regex::new(r"varchar\((\d+)\)").unwrap();
    for (key, value) in column {
        let expected_type = table_map.get(key);
        if expected_type.is_none() {
            anyhow::bail!(
                "Column {} not found in type check map for table {}",
                key,
                table_name
            );
        }
        let expected_type = expected_type.unwrap();

        // Check for int(11) type
        if expected_type.contains("int(") {
            // Check if value matches @\w+id pattern
            let value_str = value.as_str().unwrap_or("");

            if !id_pattern.is_match(value_str) {
                // Try to cast to i64
                if value.as_i64().is_none() {
                    anyhow::bail!(
                        "Column {} in table {} expects int(11), but value '{}' is {}",
                        key,
                        table_name,
                        value_str,
                        "neither a reference (@\\w+id) nor a valid integer"
                    );
                }
            }
        } else if expected_type.contains("float") {
            if value.as_f64().is_none() {
                anyhow::bail!(
                    "Column {} in table {} expects {}, but value cannot be cast to float",
                    key,
                    table_name,
                    expected_type
                );
            }
        } else if let Some(caps) = varbinary_pattern.captures(expected_type) {
            let max_length: usize = caps.get(1).unwrap().as_str().parse().unwrap();

            if let Some(value_str) = value.as_str() {
                if !value_str.chars().all(|c| "0123456789abcdef".contains(c)) {
                    anyhow::bail!(
                        "Column {} in table {} expects {}, but string value '{}' contains invalid hex characters",
                        key,
                        table_name,
                        expected_type,
                        value_str
                    );
                }
                if value_str.len() * 4 >= max_length {
                    anyhow::bail!(
                        "Column {} in table {} expects {}, but string value '{}' has length {} * 4 = {} >= {}",
                        key,
                        table_name,
                        expected_type,
                        value_str,
                        value_str.len(),
                        value_str.len() * 4,
                        max_length
                    );
                }
            } else {
                anyhow::bail!(
                    "Column {} in table {} expects {}, but value cannot be cast to str",
                    key,
                    table_name,
                    expected_type
                );
            }
        } else if let Some(caps) = varchar_pattern.captures(expected_type) {
            let max_length: usize = caps.get(1).unwrap().as_str().parse().unwrap();

            if let Some(value_str) = value.as_str() {
                if value_str.len() >= max_length {
                    anyhow::bail!(
                        "Column {} in table {} expects {}, but string value '{}' has length {} >= {}",
                        key,
                        table_name,
                        expected_type,
                        value_str,
                        value_str.len(),
                        max_length
                    );
                }
            } else {
                anyhow::bail!(
                    "Column {} in table {} expects {}, but value cannot be cast to str",
                    key,
                    table_name,
                    expected_type
                );
            }
        }
    }
    Ok(())
}
