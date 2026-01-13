#[cfg(test)]
mod tests {
    use crate::checktypes::check_type;
    use crate::cmdline::CliArgs;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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
        let value = serde_yaml::Value::String(long_string);
        let result = check_type("runs", &"compiler".to_string(), &value, &sqltypes);
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

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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
        let value = serde_yaml::Value::Number(serde_yaml::Number::from(10000000)); // Exceeds int(8) range
        let result = check_type("runs", &"nodes".to_string(), &value, &sqltypes);
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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
        let value = serde_yaml::Value::String(long_binary);
        let result = check_type("tasks", &"affinity".to_string(), &value, &sqltypes);
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
