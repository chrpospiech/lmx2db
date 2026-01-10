#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::sqltypes::create_sqltype_file;
    use crate::sqltypes::sqltype_hashmap::check_sqltypes_file;
    use sqlx::{MySql, Pool};
    use tempfile::NamedTempFile;

    #[sqlx::test(fixtures("tests/fixtures/lmxtest.sql"))]
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
