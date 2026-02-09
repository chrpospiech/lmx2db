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

#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::create_sql::create_import_statement;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_create_import_statement(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args)
            .await
            .expect("Failed to read sqltypes");

        let keys = vec![
            "rid".to_string(),
            "compiler".to_string(),
            "nodes".to_string(),
        ];
        let values = vec![vec![
            serde_yaml::Value::String("@rid".to_string()),
            serde_yaml::Value::String("gcc".to_string()),
            serde_yaml::Value::Number(serde_yaml::Number::from(16)),
        ]];

        let sql = create_import_statement("runs", &keys, &values, &sqltypes)?;
        assert_eq!(
            sql,
            "INSERT INTO runs (rid, compiler, nodes) VALUES\n(@rid, 'gcc', 16);"
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_create_import_statement_multi_row(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args)
            .await
            .expect("Failed to read sqltypes");

        let keys = vec![
            "rid".to_string(),
            "compiler".to_string(),
            "nodes".to_string(),
        ];
        let values = vec![
            vec![
                serde_yaml::Value::String("@rid".to_string()),
                serde_yaml::Value::String("gcc".to_string()),
                serde_yaml::Value::Number(serde_yaml::Number::from(16)),
            ],
            vec![
                serde_yaml::Value::String("@rid".to_string()),
                serde_yaml::Value::String("icc".to_string()),
                serde_yaml::Value::Number(serde_yaml::Number::from(32)),
            ],
            vec![
                serde_yaml::Value::String("@rid".to_string()),
                serde_yaml::Value::String("clang".to_string()),
                serde_yaml::Value::Number(serde_yaml::Number::from(64)),
            ],
        ];

        let sql = create_import_statement("runs", &keys, &values, &sqltypes)?;
        assert_eq!(
            sql,
            "INSERT INTO runs (rid, compiler, nodes) VALUES\n(@rid, 'gcc', 16),\n(@rid, 'icc', 32),\n(@rid, 'clang', 64);"
        );
        Ok(())
    }
}
