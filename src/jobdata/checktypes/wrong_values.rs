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
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_wrong_string_length(pool: Pool<MySql>) {
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
        let long_string = "a".repeat(40); // Assuming max length is less than 32
        let tuple = [(
            "compiler".to_string(),
            serde_yaml::Value::String(long_string),
        )];
        let result = check_type("runs", &tuple, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Column compiler in table runs expects varchar(32), but string value '{}' has length {} >= 32",
                "a".repeat(40),
                40
            )
        );
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_int_value(pool: Pool<MySql>) {
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
        let tuple = [(
            "nodes".to_string(),
            serde_yaml::Value::Number(serde_yaml::Number::from(10000000)),
        )];
        let result = check_type("runs", &tuple, &sqltypes);
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_varbinary_length(pool: Pool<MySql>) {
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
        let long_binary = "a".repeat(1030); // Assuming max length is less than 4096
        let tuple = [(
            "affinity".to_string(),
            serde_yaml::Value::String(long_binary),
        )];
        let result = check_type("tasks", &tuple, &sqltypes);
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
    }
}
