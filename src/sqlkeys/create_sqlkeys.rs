#[cfg(test)]
mod tests {
    use crate::sqlkeys::create_sqlkey_file;
    use crate::sqlkeys::sqlkey_hashmap::check_sqlkeys_file;
    use crate::sqlkeys::CliArgs;
    use sqlx::{MySql, Pool};
    use tempfile::NamedTempFile;

    #[sqlx::test(fixtures("tests/fixtures/lmxtest.sql"))]
    async fn test_create_sqlkey_file_with_pool(pool: Pool<MySql>) {
        // Create temporary file for output
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let args = CliArgs {
            sqlkeys_file: temp_path,
            verbose: false,
            dry_run: false,
            create_sqlkeys: true,
            db_url: String::new(), // Not needed when using injected pool
            sql_file: "import.sql".to_string(),
            transaction_per_job: false,
            module_db_file: "moduledefs.db".to_string(),
            settings_file: "settings.yml".to_string(),
            project_file: "project.yml".to_string(),
            files: vec![],
        };

        // Use the injected pool directly

        create_sqlkey_file(Some(pool), &args)
            .await
            .expect("Failed to create sqlkey file");

        // Verify the contents of the created sqlkey file
        assert!(
            check_sqlkeys_file(args.sqlkeys_file.clone()),
            "SQL key values do not match expected structure"
        );

        // Clean up temporary file
        std::fs::remove_file(&args.sqlkeys_file).unwrap();
    }

    #[tokio::test]
    async fn test_create_sqlkey_file_without_pool() {
        // Test error handling when pool is None
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let args = CliArgs {
            sqlkeys_file: temp_path,
            verbose: false,
            dry_run: false,
            create_sqlkeys: true,
            db_url: "mysql://lmxtest:lmxtest@localhost/lmxtest".to_string(),
            sql_file: "import.sql".to_string(),
            transaction_per_job: false,
            module_db_file: "moduledefs.db".to_string(),
            settings_file: "settings.yml".to_string(),
            project_file: "project.yml".to_string(),
            files: vec![],
        };

        // No database pool provided
        let pool: Option<Pool<MySql>> = None;

        let result = create_sqlkey_file(pool, &args).await;
        assert!(
            result.is_err(),
            "Expected error when no database pool is provided"
        );
    }
}
