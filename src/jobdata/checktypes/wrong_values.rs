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
    use crate::jobdata::checktypes::check_type;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_wrong_string_length(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;
        let long_string = "a".repeat(40); // Assuming max length is less than 32
        let keys = vec!["compiler".to_string()];
        let values = vec![vec![serde_yaml::Value::String(long_string)]];
        let result = check_type("runs", &keys, &values, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Column compiler in table runs expects varchar(32), but string value '{}' has length {} >= 32",
                "a".repeat(40),
                40
            )
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_int_value(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;
        let keys = vec!["nodes".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            10000000,
        ))]];
        let result = check_type("runs", &keys, &values, &sqltypes);
        assert!(result.is_ok());
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_varbinary_length(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;
        let long_binary = "a".repeat(1030); // Assuming max length is less than 4096
        let keys = vec!["affinity".to_string()];
        let values = vec![vec![serde_yaml::Value::String(long_binary)]];
        let result = check_type("tasks", &keys, &values, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Column affinity in table tasks expects varbinary(4096), but string value '{}' has length {} * 4 = {} >= 4096",
                "a".repeat(1030),
                1030,
                1030*4
            )
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_null_value(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;
        let keys = vec!["elapsed".to_string()];
        let values = vec![vec![serde_yaml::Value::Null]];
        let result = check_type("runs", &keys, &values, &sqltypes);
        assert!(result.is_err());
        let expected_msg =
            "Column elapsed in table runs expects float, but value cannot be cast to float";
        assert_eq!(result.unwrap_err().to_string(), expected_msg);
        Ok(())
    }
}
