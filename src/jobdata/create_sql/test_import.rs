#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::create_sql::create_import_statement;
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    async fn test_create_import_statement(pool: Pool<MySql>) -> Result<()> {
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
                "rid".to_string(),
                serde_yaml::Value::String("@rid".to_string()),
            ),
            (
                "compiler".to_string(),
                serde_yaml::Value::String("gcc".to_string()),
            ),
            (
                "nodes".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(16)),
            ),
        ];

        let sql = create_import_statement("runs", &tuple, &sqltypes)?;
        assert_eq!(
            sql,
            "INSERT INTO runs (rid, compiler, nodes) VALUES (@rid, 'gcc', 16);"
        );
        Ok(())
    }
}
