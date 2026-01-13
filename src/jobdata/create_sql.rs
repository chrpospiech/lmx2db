use crate::jobdata::checktypes::check_type;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

#[cfg(test)]
pub(crate) mod test_import;

pub fn create_import_statement(
    table_name: &str,
    column: &[(String, serde_yaml::Value)],
    sqltypes: &SqlTypeHashMap,
) -> Result<String> {
    // First, check types
    check_type(table_name, column, sqltypes)?;

    let columns: Vec<String> = column.iter().map(|(k, _)| k.clone()).collect();
    let values: Vec<String> = column
        .iter()
        .map(|(_, v)| match v {
            serde_yaml::Value::String(s) => format!("'{}'", s.replace("'", "''")),
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
