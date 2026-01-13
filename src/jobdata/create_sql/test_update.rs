#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::create_sql::create_update_statement;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_create_update_statement(pool: Pool<MySql>) -> Result<()> {
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

        let tuple = [
            (
                "compiler".to_string(),
                serde_yaml::Value::String("gcc-10".to_string()),
            ),
            (
                "nodes".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(32)),
            ),
        ];

        let sql = create_update_statement("runs", &tuple, "rid = @rid", &sqltypes)?;
        assert_eq!(
            sql,
            "UPDATE runs SET compiler = 'gcc-10', nodes = 32 WHERE rid = @rid;"
        );
        Ok(())
    }
}
