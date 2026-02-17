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
    use crate::sqltypes::create_sqltype_file;
    use crate::sqltypes::sqltype_hashmap::check_sqltypes_file;
    use sqlx::{MySql, Pool};
    use tempfile::NamedTempFile;

    #[sqlx::test(fixtures(
        "../../tests/fixtures/tables.sql",
        "../../tests/fixtures/functs4test.sql"
    ))]
    async fn test_create_sqltype_file_with_pool(pool: Pool<MySql>) {
        // Create temporary file for output
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let args = CliArgs {
            sqltypes_file: temp_path,
            verbose: false,
            dry_run: false,
            create_sqltypes: true,
            db_url: String::new(), // Not needed when using injected pool
            ..Default::default()
        };

        // Use the injected pool directly

        create_sqltype_file(Some(pool), &args)
            .await
            .expect("Failed to create sqltype file");

        // Verify the contents of the created sqltype file
        assert!(
            check_sqltypes_file(args.sqltypes_file.clone()),
            "SQL key values do not match expected structure"
        );

        // Clean up temporary file
        std::fs::remove_file(&args.sqltypes_file).unwrap();
    }

    #[tokio::test]
    async fn test_create_sqltype_file_without_pool() {
        // Test error handling when pool is None
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let args = CliArgs {
            sqltypes_file: temp_path,
            verbose: false,
            dry_run: false,
            create_sqltypes: true,
            db_url: "mysql://lmxtest:lmxtest@localhost/lmxtest".to_string(),
            ..Default::default()
        };

        // No database pool provided
        let pool: Option<Pool<MySql>> = None;

        let result = create_sqltype_file(pool, &args).await;
        assert!(
            result.is_err(),
            "Expected error when no database pool is provided"
        );
    }
}
