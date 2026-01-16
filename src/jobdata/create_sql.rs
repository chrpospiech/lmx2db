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
    column: &[(String, serde_yaml::Value)],
    sqltypes: &SqlTypeHashMap,
) -> Result<String> {
    // First, check types
    check_type(table_name, column, sqltypes)?;

    // The following regex will be used multiple times
    let id_pattern = Regex::new(r"@\w+id").unwrap();

    let columns: Vec<String> = column.iter().map(|(k, _)| k.clone()).collect();
    let values: Vec<String> = column
        .iter()
        .map(|(_, v)| match v {
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

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_name,
        columns.join(", "),
        values.join(", ")
    );
    Ok(sql)
}

pub fn create_update_statement(
    table_name: &str,
    column: &[(String, serde_yaml::Value)],
    where_clause: &str,
    sqltypes: &SqlTypeHashMap,
) -> Result<String> {
    // First, check types
    check_type(table_name, column, sqltypes)?;

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
