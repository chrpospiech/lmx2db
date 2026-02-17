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
    use crate::{
        cmdline::CliArgs, jobdata::table_environ::import_into_environ_table,
        sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;
    use std::collections::HashMap;

    /// Test successful import when environ section exists with valid data
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_with_valid_data(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        // Use the GROMACS test data which has an environ section
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");

        // Read the LMX summary file
        let lmx_summary: crate::jobdata::LmxSummary =
            serde_yaml::from_str(&std::fs::read_to_string(&lmx_file)?)?;

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return a comment line and exactly one query
        assert_eq!(
            queries.len(),
            2,
            "Expected a comment line and one query for environ import"
        );

        // Verify the query contains expected structure
        let query = &queries[1];
        assert!(
            query.contains("INSERT"),
            "Query should be an INSERT statement"
        );
        assert!(
            query.contains("environ"),
            "Query should insert into environ table"
        );

        // Check that some expected environment variables are present
        assert!(
            query.contains("BASH_ENV") || query.contains("CMAKE_PREFIX_PATH"),
            "Query should contain environment variable names from test data"
        );

        Ok(())
    }

    /// Test with mixed string and sequence values
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_mixed_values(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        // Create an LMX summary with mixed environ values
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut environ_section = HashMap::new();

        // Add string value
        environ_section.insert(
            "SIMPLE_VAR".to_string(),
            serde_yaml::Value::String("simple_value".to_string()),
        );

        // Add sequence value
        environ_section.insert(
            "PATH_VAR".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("/usr/bin:".to_string()),
                serde_yaml::Value::String("/usr/local/bin".to_string()),
            ]),
        );

        lmx_summary.insert("environ".to_string(), environ_section);

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return a comment line and exactly one query
        assert_eq!(
            queries.len(),
            2,
            "Expected a comment line and one query for environ import"
        );

        let query = &queries[1];

        // Verify both environment variables are included
        assert!(
            query.contains("SIMPLE_VAR"),
            "Query should contain SIMPLE_VAR"
        );
        assert!(query.contains("PATH_VAR"), "Query should contain PATH_VAR");

        // Verify simple string value is present
        assert!(
            query.contains("simple_value"),
            "Query should contain simple_value"
        );

        // Verify sequence is joined correctly
        assert!(
            query.contains("/usr/bin:/usr/local/bin"),
            "Query should contain joined sequence value"
        );

        Ok(())
    }

    /// Test that verbose mode doesn't prevent importing of data
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_verbose_mode(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: true,
            ..Default::default()
        };

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        // Use the GROMACS test data which has an environ section
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");

        // Read the LMX summary file
        let lmx_summary: crate::jobdata::LmxSummary =
            serde_yaml::from_str(&std::fs::read_to_string(&lmx_file)?)?;

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return a comment line and exactly one query
        assert_eq!(
            queries.len(),
            2,
            "Expected a comment line and one query for environ import"
        );

        // Verify the query contains expected structure
        let query = &queries[1];
        assert!(
            query.contains("INSERT"),
            "Query should be an INSERT statement"
        );
        assert!(
            query.contains("environ"),
            "Query should insert into environ table"
        );

        // Check that some expected environment variables are present
        assert!(
            query.contains("BASH_ENV") || query.contains("CMAKE_PREFIX_PATH"),
            "Query should contain environment variable names from test data"
        );

        Ok(())
    }
}
