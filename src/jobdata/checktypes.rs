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

/// Retrieves the expected SQL types for the specified keys in a given table.
///
/// # Arguments
/// * `table_name` - The name of the table to look up
/// * `keys` - A slice of column names for which to retrieve types
/// * `sqltypes` - A reference to the SQL type mapping
///
/// Returns a vector of expected SQL types corresponding to the provided keys.
///
/// # Errors
/// Returns an error if the table or any of the keys are not found in the type mapping
///
pub fn get_types(
    table_name: &str,
    keys: &[String],
    sqltypes: &SqlTypeHashMap,
) -> Result<Vec<String>> {
    let table_map = sqltypes.get(table_name);
    if table_map.is_none() {
        anyhow::bail!("Table {} not found in type check map", table_name);
    }
    let table_map = table_map.unwrap();

    // Pre-compute expected types for all keys
    let mut expected_types: Vec<String> = Vec::new();
    for key in keys.iter() {
        let expected_type = table_map.get(key);
        if expected_type.is_none() {
            anyhow::bail!(
                "Column {} not found in type check map for table {}",
                key,
                table_name
            );
        }
        expected_types.push(expected_type.unwrap().clone());
    }
    Ok(expected_types)
}

/// Checks whether the provided values conform to the expected SQL types
/// for the specified keys in a given table.
///
/// # Arguments
/// * `table_name` - The name of the table to check against
/// * `keys` - A slice of column names corresponding to the values
/// * `types` - A slice of expected SQL types expected for each provided value row
/// * `values` - A slice of value rows to validate
///
/// `table_name` and `keys` are only needed for error reporting.
/// They do not influence the type checking logic.
///
/// # Errors
/// Returns an error if any value does not conform to its expected SQL type
///
pub fn check_types(
    table_name: &str,
    keys: &[String],
    types: &[String],
    values: &[Vec<serde_yaml::Value>],
) -> Result<()> {
    // The following regexes will be used multiple times
    let id_pattern = Regex::new(r"@\w+id").unwrap();
    let varbinary_pattern = Regex::new(r"varbinary\((\d+)\)").unwrap();
    let varchar_pattern = Regex::new(r"varchar\((\d+)\)").unwrap();

    for value_row in values {
        if value_row.len() != types.len() {
            anyhow::bail!(
                "Row length mismatch in table {}: expected {} columns, got {}",
                table_name,
                types.len(),
                value_row.len()
            );
        }
        for (i, expected_type) in types.iter().enumerate() {
            let value = &value_row[i];

            // Check for int(11) type
            if expected_type.contains("int(") {
                // Check if value matches @\w+id pattern
                let value_str = try_cast_into_string(value).unwrap_or_default();

                if !id_pattern.is_match(&value_str) {
                    // Try to cast to i64
                    if value.as_i64().is_none() {
                        anyhow::bail!(
                            "Column {} in table {} expects int(11), but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                            keys[i],
                            table_name,
                            value_str
                        );
                    }
                }
            } else if expected_type.contains("float") {
                if value.as_f64().is_none() {
                    anyhow::bail!(
                        "Column {} in table {} expects {}, but value cannot be cast to float",
                        keys[i],
                        table_name,
                        expected_type
                    );
                }
            } else if let Some(caps) = varbinary_pattern.captures(expected_type) {
                let max_length: usize = caps.get(1).unwrap().as_str().parse().unwrap();

                if let Ok(value_str) = try_cast_into_string(value) {
                    if !value_str.chars().all(|c| "0123456789abcdef".contains(c)) {
                        anyhow::bail!(
                            "Column {} in table {} expects {}, but string value '{}' contains invalid hex characters",
                            keys[i],
                            table_name,
                            expected_type,
                            value_str
                        );
                    }
                    if value_str.len() * 4 >= max_length {
                        anyhow::bail!(
                            "Column {} in table {} expects {}, but string value '{}' has length {} * 4 = {} >= {}",
                            keys[i],
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
                        keys[i],
                        table_name,
                        expected_type
                    );
                }
            } else if let Some(caps) = varchar_pattern.captures(expected_type) {
                let max_length: usize = caps.get(1).unwrap().as_str().parse().unwrap();

                if let Ok(value_str) = try_cast_into_string(value) {
                    if value_str.len() >= max_length {
                        anyhow::bail!(
                            "Column {} in table {} expects {}, but string value '{}' has length {} >= {}",
                            keys[i],
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
                        keys[i],
                        table_name,
                        expected_type
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn try_cast_into_string(value: &serde_yaml::Value) -> Result<String> {
    match value {
        serde_yaml::Value::String(s) => Ok(s.clone()),
        serde_yaml::Value::Number(n) => Ok(n.to_string()),
        serde_yaml::Value::Bool(b) => Ok(if *b { "1".to_string() } else { "0".to_string() }),
        serde_yaml::Value::Null => anyhow::bail!("Cannot cast null value to string"),
        _ => anyhow::bail!("Cannot cast value to string: unsupported type"),
    }
}
