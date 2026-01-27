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

use crate::jobdata::checktypes::{check_types, get_types, try_cast_into_string};
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

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
    let types: Vec<String> = get_types(table_name, keys, sqltypes)?;
    check_types(table_name, keys, &types, values)?;

    let value_rows: Vec<String> = values
        .iter()
        .map(|value_row| {
            let row_values: Vec<String> = value_row
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let v_string = try_cast_into_string(v)?;
                    let v_final = if types[i].contains("varbinary")
                        || types[i].contains("varchar")
                        || types[i].contains("binary")
                    {
                        format!("'{}'", v_string.replace("'", "''"))
                    } else {
                        v_string
                    };
                    Ok(v_final)
                })
                .collect::<Result<Vec<String>>>()?;
            Ok(format!("({})", row_values.join(", ")))
        })
        .collect::<Result<Vec<String>>>()?;

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
    let types: Vec<String> = get_types(table_name, &keys, sqltypes)?;
    check_types(table_name, &keys, &types, &values)?;

    let set_clauses: Vec<String> = column
        .iter()
        .enumerate()
        .map(|(i, (k, v))| {
            let v_string = try_cast_into_string(v)?;
            let v_final = if types[i].contains("varbinary")
                || types[i].contains("varchar")
                || types[i].contains("binary")
            {
                format!("'{}'", v_string.replace("'", "''"))
            } else {
                v_string
            };
            Ok(format!("{} = {}", k, v_final))
        })
        .collect::<Result<Vec<String>>>()?;

    let sql = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name,
        set_clauses.join(",\n"),
        where_clause
    );
    Ok(sql)
}
