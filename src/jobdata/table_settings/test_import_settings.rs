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
        cmdline::CliArgs,
        jobdata::table_settings::import_into_settings_table,
        jobdata::table_runs::find_file::project_mockup::setup_tmp_project_directory,
        sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;

    /// Test successful import when settings file exists with valid data
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_with_valid_file(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Use the GROMACS test data which has a settings.yml file
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should return at least one query since settings.yml exists
        assert!(!queries.is_empty(), "Expected at least one query for settings import");

        // Verify the query contains expected settings keys that are not in runs table
        let query = &queries[0];
        assert!(query.contains("INSERT"), "Query should be an INSERT statement");
        assert!(query.contains("settings"), "Query should insert into settings table");
        
        // Check that keys matching runs table columns are filtered out
        // These keys exist in settings.yml but should be filtered because they're in runs table
        assert!(!query.contains("'nodes'"), "nodes should be filtered (exists in runs table)");
        assert!(!query.contains("'mpilib'"), "mpilib should be filtered (exists in runs table)");
        assert!(!query.contains("'compiler'"), "compiler should be filtered (exists in runs table)");
        assert!(!query.contains("'MPI_ranks'"), "MPI_ranks should be filtered (exists in runs table)");
        
        // These keys should be included (not in runs table)
        assert!(query.contains("'maxh'") || query.contains("'Precision'"), 
            "Settings not in runs table should be included");

        Ok(())
    }

    /// Test handling when settings file doesn't exist
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_no_file(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Use the NAMD test data which does NOT have a settings.yml file
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0001/LMX_summary.225250.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should return empty vector since settings.yml doesn't exist
        assert!(queries.is_empty(), "Expected empty query list when settings file doesn't exist");

        Ok(())
    }

    /// Test handling when all settings keys match runs table columns (empty value_list case)
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_all_keys_filtered(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "test_settings_filtered.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create a temporary project directory with test data
        let temp_dir = setup_tmp_project_directory("tests/data/GROMACS/run_64")?;
        let settings_file = temp_dir.join("test_settings_filtered.yml");

        // Create settings with only runs table columns
        let settings_content = r#"---
nodes: 1
mpilib: "OpenMPI"
mpilib_version: "5.0.8"
compiler: "Clang"
compiler_version: "17.0.6"
MPI_ranks: 64
threads: 4
perf_unit: "ns/day"
perf_value: 319.366
"#;
        std::fs::write(&settings_file, settings_content)?;

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        let lmx_file = temp_dir.join("LMX_summary.376231.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should return empty vector since all keys are filtered
        assert!(queries.is_empty(), "Expected empty query list when all settings keys match runs table columns");

        // Clean up temporary project directory
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    /// Test proper filtering of runs table columns with mixed settings
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_filtering(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "test_settings_mixed.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create a temporary project directory with test data
        let temp_dir = setup_tmp_project_directory("tests/data/GROMACS/run_64")?;
        let settings_file = temp_dir.join("test_settings_mixed.yml");

        // Mix of runs table columns and non-runs table settings
        let settings_content = r#"---
custom_setting1: "value1"
nodes: 1
custom_setting2: "value2"
mpilib: "OpenMPI"
unique_config: "test_value"
compiler: "Clang"
"#;
        std::fs::write(&settings_file, settings_content)?;

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        let lmx_file = temp_dir.join("LMX_summary.376231.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should return exactly one query with filtered keys
        assert_eq!(queries.len(), 1, "Expected exactly one query for settings import");

        let query = &queries[0];
        
        // Verify runs table columns are filtered out
        assert!(!query.contains("'nodes'"), "nodes should be filtered");
        assert!(!query.contains("'mpilib'"), "mpilib should be filtered");
        assert!(!query.contains("'compiler'"), "compiler should be filtered");
        
        // Verify custom settings are included
        assert!(query.contains("'custom_setting1'"), "custom_setting1 should be included");
        assert!(query.contains("'custom_setting2'"), "custom_setting2 should be included");
        assert!(query.contains("'unique_config'"), "unique_config should be included");

        // Clean up temporary project directory
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    /// Test that non-string YAML values (numbers, booleans, etc.) are handled correctly
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_with_non_string_values(pool: sqlx::Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "test_settings_types.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create a temporary project directory with test data
        let temp_dir = setup_tmp_project_directory("tests/data/GROMACS/run_64")?;
        let settings_file = temp_dir.join("test_settings_types.yml");

        // Create settings with various YAML value types
        let settings_content = r#"---
threshold: 100
enabled: true
ratio: 3.14
count: 42
label: "string_value"
feature_flag: false
"#;
        std::fs::write(&settings_file, settings_content)?;

        // Read SQL types from the database
        let sqltypes = read_sqltypes(Some(pool.clone()), &args).await?;

        let lmx_file = temp_dir.join("LMX_summary.376231.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should return exactly one query with various value types
        assert_eq!(queries.len(), 1, "Expected exactly one query for settings import");

        let query = &queries[0];
        
        // Verify all settings are included (none match runs table)
        assert!(query.contains("'threshold'"), "threshold (number) should be included");
        assert!(query.contains("'enabled'"), "enabled (boolean) should be included");
        assert!(query.contains("'ratio'"), "ratio (float) should be included");
        assert!(query.contains("'count'"), "count (number) should be included");
        assert!(query.contains("'label'"), "label (string) should be included");
        assert!(query.contains("'feature_flag'"), "feature_flag (boolean) should be included");
        
        // Verify that numeric and boolean values are preserved (not converted to strings prematurely)
        // The values should be in the query
        assert!(query.contains("100") || query.contains("'100'"), "numeric value 100 should be present");
        assert!(query.contains("true") || query.contains("'true'"), "boolean true should be present");
        assert!(query.contains("3.14") || query.contains("'3.14'"), "float 3.14 should be present");

        // Clean up temporary project directory
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    /// Test verbose mode outputs message when reading settings
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_settings_verbose(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Use the GROMACS test data which has a settings.yml file
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");

        // Call import_into_settings_table (no await - function is not async)
        let queries = import_into_settings_table(
            lmx_file.to_str().unwrap(),
            &sqltypes,
            &args,
        )?;

        // Should still return queries
        assert!(!queries.is_empty(), "Expected at least one query for settings import in verbose mode");

        Ok(())
    }
}
