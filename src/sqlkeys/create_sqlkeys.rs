#[cfg(test)]
mod tests {
    use crate::sqlkeys::create_sqlkey_file;
    use crate::sqlkeys::CliArgs;
    use sqlx::mysql::MySqlPool;
    use sqlx::{MySql, Pool};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create_sqlkey_file_with_pool() {
        // Create temporary file for output
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let args = CliArgs {
            sqlkeys_file: temp_path,
            verbose: false,
            dry_run: false,
            create_sqlkeys: true,
            db_url: "mysql://lmxtest:lmxtest@localhost/lmxtest".to_string(),
        };

        // Connect to the test database. Panic if it is not available.
        let database_url: String = args.db_url.clone();
        let pool: Pool<MySql> = MySqlPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        // Test would call create_sqlkey_file
        create_sqlkey_file(Some(pool), &args)
            .await
            .expect("Failed to create sqlkey file");

        // Verify file contents after
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
