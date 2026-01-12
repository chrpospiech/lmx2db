#[cfg(test)]
mod tests {
    use crate::checktypes::check_type;
    use crate::cmdline::CliArgs;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../sqltypes/tests/fixtures/lmxtest.sql"))]
    async fn test_wrong_table_or_key(pool: Pool<MySql>) {
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

        let value = serde_yaml::Value::String("test".to_string());

        // Test for non-existing table
        let result = check_type(
            "non_existing_table",
            &"some_key".to_string(),
            &value,
            &sqltypes,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Table non_existing_table not found in type check map"
        );

        // Test for non-existing key
        let result = check_type("runs", &"non_existing_key".to_string(), &value, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Column non_existing_key not found in type check map for table runs"
        );
    }

    #[sqlx::test(fixtures("../sqltypes/tests/fixtures/lmxtest.sql"))]
    pub async fn test_wrong_foreign_key(pool: Pool<MySql>) {
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

        let value = serde_yaml::Value::String("not_an_id".to_string());

        // Test for foreign key that is not an @\w+id reference and not an integer
        let result = check_type("runs", &"clid".to_string(), &value, &sqltypes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Column clid in table runs expects int(11), but value 'not_an_id' is neither a reference (@\\w+id) nor a valid integer"
        );
    }

    #[sqlx::test(fixtures("../sqltypes/tests/fixtures/lmxtest.sql"))]
    pub async fn test_correct_foreign_key(pool: Pool<MySql>) {
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

        // Test for foreign key that is a valid @\w+id reference
        let value_ref = serde_yaml::Value::String("@clid".to_string());
        let result_ref = check_type("runs", &"clid".to_string(), &value_ref, &sqltypes);
        assert!(result_ref.is_ok());

        // Test for foreign key that is a valid integer
        let value_int = serde_yaml::Value::Number(serde_yaml::Number::from(12345));
        let result_int = check_type("runs", &"clid".to_string(), &value_int, &sqltypes);
        assert!(result_int.is_ok());
    }
}
