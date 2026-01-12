use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;
use regex::Regex;

#[cfg(test)]
pub(crate) mod elementary;
#[cfg(test)]
pub(crate) mod wrong_values;

pub fn check_type(
    table_name: &str,
    key: &String,
    value: &serde_yaml::Value,
    map: &SqlTypeHashMap,
) -> Result<()> {
    let _ = value;
    let table_map = map.get(table_name);
    if table_map.is_none() {
        anyhow::bail!("Table {} not found in type check map", table_name);
    }
    let table_map = table_map.unwrap();
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
        let id_pattern = Regex::new(r"@\w+id").unwrap();
        let value_str = value.as_str().unwrap_or("");

        if !id_pattern.is_match(value_str) {
            // Try to cast to i64
            if value.as_i64().is_none() {
                anyhow::bail!(
                    "Column {} in table {} expects int(11), but value '{}' is neither a reference (@\\w+id) nor a valid integer",
                    key,
                    table_name,
                    value_str
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
    } else if let Some(caps) = Regex::new(r"varbinary\((\d+)\)")
        .unwrap()
        .captures(expected_type)
    {
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
    } else if let Some(caps) = Regex::new(r"varchar\((\d+)\)")
        .unwrap()
        .captures(expected_type)
    {
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
    Ok(())
}
