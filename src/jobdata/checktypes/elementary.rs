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
    async fn test_wrong_table_or_key(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;

        let tuple = [("non_existing_key".to_string(), serde_yaml::Value::Null)];

        // Test for non-existing table
        let result = check_type("non_existing_table", &tuple, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Table non_existing_table not found in type check map"
        );

        // Test for non-existing table
        let result = check_type("runs", &tuple, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Column non_existing_key not found in type check map for table runs"
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_wrong_foreign_key(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;

        let tuple = [(
            "clid".to_string(),
            serde_yaml::Value::String("not_an_id".to_string()),
        )];

        // Test for foreign key that is not an @\w+id reference and not an integer
        let result = check_type("runs", &tuple, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Column clid in table runs expects int(11), but value 'not_an_id' is neither a reference (@\\w+id) nor a valid integer"
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_correct_foreign_key(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            create_sqltypes: false,
            db_url: String::new(),
            ..Default::default()
        };
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool), &args).await?;

        // Test for foreign key that is a valid @\w+id reference
        let tuple_ref = [(
            "clid".to_string(),
            serde_yaml::Value::String("@clid".to_string()),
        )];
        let result_ref = check_type("runs", &tuple_ref, &sqltypes);
        assert!(result_ref.is_ok());

        // Test for foreign key that is a valid integer
        let tuple_int = [(
            "clid".to_string(),
            serde_yaml::Value::Number(serde_yaml::Number::from(12345)),
        )];
        let result_int = check_type("runs", &tuple_int, &sqltypes);
        assert!(result_int.is_ok());
        Ok(())
    }
}
