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
use anyhow::{bail, Result};
use regex::Regex;

#[cfg(test)]
pub(crate) mod elementary;
#[cfg(test)]
pub(crate) mod wrong_values;
#[cfg(test)]
mod type_normalization_tests;

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
/// * `types` - A slice of expected SQL types for each provided value row
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
    let id_pattern = Regex::new(r"^@\w+id$|^[A-Za-z_]\w*_id\([^;]*\)$").unwrap();
    let varbinary_pattern = Regex::new(r"varbinary\((\d+)\)").unwrap();
    let varchar_pattern = Regex::new(r"varchar\((\d+)\)").unwrap();

    for value_row in values {
        if value_row.len() != types.len() {
            bail!(
                "Row length mismatch in table {}: expected {} columns, got {}",
                table_name,
                types.len(),
                value_row.len()
            );
        }
        for (i, expected_type) in types.iter().enumerate() {
            let value = &value_row[i];
            
            // Normalize type string to lowercase for case-insensitive matching
            let expected_type_lower = expected_type.to_lowercase();
            let is_unsigned = expected_type_lower.contains("unsigned");

            // Check for integer types: bigint, tinyint, smallint, int (order matters!)
            // Check most specific types first (tinyint, smallint) before generic int
            if expected_type_lower.contains("bigint") {
                // BIGINT types: bigint(20) or bigint(20) unsigned
                if is_unsigned {
                    // unsigned bigint: 0 to u64::MAX
                    if value.as_u64().is_none() {
                        bail!(
                            "Column {} in table {} expects unsigned type {}, but value cannot be cast to unsigned integer",
                            keys[i],
                            table_name,
                            expected_type
                        );
                    }
                    // No range check needed - u64 is the max range
                } else {
                    // signed bigint: i64::MIN to i64::MAX
                    let value_str = try_cast_into_string(value).unwrap_or_default();
                    if !id_pattern.is_match(&value_str) {
                        if value.as_i64().is_none() {
                            bail!(
                                "Column {} in table {} expects bigint, but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                                keys[i],
                                table_name,
                                value_str
                            );
                        }
                        // No range check needed - i64 is the max range for signed bigint
                    }
                }
            } else if expected_type_lower.contains("tinyint") {
                // TINYINT types: tinyint(4) or tinyint(4) unsigned
                if is_unsigned {
                    // unsigned tinyint: 0 to u8::MAX
                    if value.as_u64().is_none() {
                        bail!(
                            "Column {} in table {} expects unsigned type {}, but value cannot be cast to unsigned integer",
                            keys[i],
                            table_name,
                            expected_type
                        );
                    }
                    let intval = value.as_u64().unwrap();
                    if intval > u8::MAX as u64 {
                        bail!(
                            "Value {} is out of unsigned tinyint range ({}..={})",
                            intval,
                            0,
                            u8::MAX
                        );
                    }
                } else {
                    // signed tinyint: i8::MIN to i8::MAX
                    let value_str = try_cast_into_string(value).unwrap_or_default();
                    if !id_pattern.is_match(&value_str) {
                        if value.as_i64().is_none() {
                            bail!(
                                "Column {} in table {} expects tinyint, but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                                keys[i],
                                table_name,
                                value_str
                            );
                        }
                        let intval = value.as_i64().unwrap();
                        if intval < i8::MIN as i64 || intval > i8::MAX as i64 {
                            bail!(
                                "Value {} is out of signed tinyint range ({}..={})",
                                intval,
                                i8::MIN,
                                i8::MAX
                            );
                        }
                    }
                }
            } else if expected_type_lower.contains("smallint") {
                // SMALLINT types: smallint(6) or smallint(6) unsigned
                if is_unsigned {
                    // unsigned smallint: 0 to u16::MAX
                    if value.as_u64().is_none() {
                        bail!(
                            "Column {} in table {} expects unsigned type {}, but value cannot be cast to unsigned integer",
                            keys[i],
                            table_name,
                            expected_type
                        );
                    }
                    let intval = value.as_u64().unwrap();
                    if intval > u16::MAX as u64 {
                        bail!(
                            "Value {} is out of unsigned smallint range ({}..={})",
                            intval,
                            0,
                            u16::MAX
                        );
                    }
                } else {
                    // signed smallint: i16::MIN to i16::MAX
                    let value_str = try_cast_into_string(value).unwrap_or_default();
                    if !id_pattern.is_match(&value_str) {
                        if value.as_i64().is_none() {
                            bail!(
                                "Column {} in table {} expects smallint, but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                                keys[i],
                                table_name,
                                value_str
                            );
                        }
                        let intval = value.as_i64().unwrap();
                        if intval < i16::MIN as i64 || intval > i16::MAX as i64 {
                            bail!(
                                "Value {} is out of signed smallint range ({}..={})",
                                intval,
                                i16::MIN,
                                i16::MAX
                            );
                        }
                    }
                }
            } else if expected_type_lower.contains("int(") {
                // INT types: int(11) or int(11) unsigned
                if is_unsigned {
                    // unsigned int: 0 to u32::MAX
                    if value.as_u64().is_none() {
                        bail!(
                            "Column {} in table {} expects unsigned type {}, but value cannot be cast to unsigned integer",
                            keys[i],
                            table_name,
                            expected_type
                        );
                    }
                    let intval = value.as_u64().unwrap();
                    if intval > u32::MAX as u64 {
                        bail!(
                            "Interval timer profiler ticks value {} is out of u32 range ({}..={})",
                            intval,
                            0,
                            u32::MAX
                        );
                    }
                } else {
                    // signed int: i32::MIN to i32::MAX
                    let value_str = try_cast_into_string(value).unwrap_or_default();
                    if !id_pattern.is_match(&value_str) {
                        if value.as_i64().is_none() {
                            bail!(
                                "Column {} in table {} expects int(11), but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                                keys[i],
                                table_name,
                                value_str
                            );
                        }
                        let intval = value.as_i64().unwrap();
                        if intval < i32::MIN as i64 || intval > i32::MAX as i64 {
                            bail!(
                                "Interval timer profiler ticks value {} is out of i32 range ({}..={})",
                                intval,
                                i32::MIN,
                                i32::MAX
                            );
                        }
                    }
                }
            } else if expected_type.contains("float") {
                if value.as_f64().is_none() {
                    bail!(
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
                        bail!(
                            "Column {} in table {} expects {}, but string value '{}' contains invalid hex characters",
                            keys[i],
                            table_name,
                            expected_type,
                            value_str
                        );
                    }
                    if value_str.len() * 4 >= max_length {
                        bail!(
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
                    bail!(
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
                        bail!(
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
                    bail!(
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

/// Attempts to convert a YAML value into a string representation.
///
/// # Arguments
/// * `value` - The YAML value to cast into a string.
///
/// # Returns
/// Returns the string representation of the input value if it is a supported
/// scalar type (string, number, or boolean).
///
/// # Errors
/// Returns an error if the value is `Null` or of an unsupported type that
/// cannot be safely converted into a string.
pub fn try_cast_into_string(value: &serde_yaml::Value) -> Result<String> {
    match value {
        serde_yaml::Value::String(s) => Ok(s.clone()),
        serde_yaml::Value::Number(n) => Ok(n.to_string()),
        serde_yaml::Value::Bool(b) => Ok(if *b { "1".to_string() } else { "0".to_string() }),
        serde_yaml::Value::Null => bail!("Cannot cast null value to string"),
        _ => bail!("Cannot cast value to string: unsupported type"),
    }
}
