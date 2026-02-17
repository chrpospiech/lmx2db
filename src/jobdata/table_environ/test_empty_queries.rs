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

    /// Test early exit when 'environ' table is not in sqltypes
    #[test]
    fn test_import_environ_missing_table() -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create an empty sqltypes map (no 'environ' table)
        let sqltypes: HashMap<String, HashMap<String, String>> = HashMap::new();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");

        // Read the LMX summary file
        let lmx_summary: crate::jobdata::LmxSummary =
            serde_yaml::from_str(&std::fs::read_to_string(&lmx_file)?)?;

        // Call import_into_environ_table with no 'environ' table in sqltypes
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return empty vector without processing the environ section
        assert!(
            queries.is_empty(),
            "Expected empty query list when 'environ' table is not in sqltypes"
        );

        Ok(())
    }

    /// Test handling when no environ section exists in LMX summary
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_no_section(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a minimal LMX summary without environ section
        let lmx_summary: crate::jobdata::LmxSummary = HashMap::new();

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return empty vector since environ section doesn't exist
        assert!(
            queries.is_empty(),
            "Expected empty query list when environ section doesn't exist"
        );

        Ok(())
    }

    /// Test handling when environ section is empty
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_empty_section(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create an LMX summary with empty environ section
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        lmx_summary.insert("environ".to_string(), HashMap::new());

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return empty vector since environ section is empty
        assert!(
            queries.is_empty(),
            "Expected empty query list when environ section is empty"
        );

        Ok(())
    }

    /// Test dry_run mode outputs message when no environ section found
    #[sqlx::test(fixtures(
        "../../../tests/fixtures/tables.sql",
        "../../../tests/fixtures/functs4test.sql"
    ))]
    pub async fn test_import_environ_dry_run_no_section(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: true,
            verbose: false,
            ..Default::default()
        };

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        // Create a minimal LMX summary without environ section
        let lmx_summary: crate::jobdata::LmxSummary = HashMap::new();

        // Call import_into_environ_table
        let queries = import_into_environ_table(&lmx_summary, &sqltypes, &args)?;

        // Should return empty vector
        assert!(
            queries.is_empty(),
            "Expected empty query list when environ section doesn't exist in dry_run mode"
        );

        Ok(())
    }
}
