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
        jobdata::table_runs::find_file::project_mockup::{
            setup_cliargs_with_project_file_name, test_import_single_lmx_file,
        },
    };
    use anyhow::Result;
    use sqlx::MySql;

    #[sqlx::test(fixtures(
        "../../tests/fixtures/lmxtest.sql",
        "../../tests/fixtures/minimal_data.sql"
    ))]
    pub async fn test_import_namd_jobdata(pool: sqlx::Pool<MySql>) -> Result<()> {
        // Create CliArgs with the specified project file that exists in the temp_dir
        let args = setup_cliargs_with_project_file_name("project.yml")?;
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/NAMD/run_0001/LMX_summary.225250.0.yml",
            &args,
        )
        .await;

        assert!(
            result.is_ok(),
            "Processing LMX file failed: {:?}",
            result.err()
        );

        // Query the database
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i32, bool, bool, u32)>(
            "SELECT `rid`, `clid`, `pid`, `ccid`, `nodes`, `has_MPItrace`, `has_iprof`, `MPI_ranks` FROM `runs`;"
        )
        .fetch_all(&pool)
        .await?;

        // Assert exactly one row was returned
        assert_eq!(
            rows.len(),
            1,
            "Expected exactly 1 row, but got {}",
            rows.len()
        );

        // Assert the values of the returned row
        let (rid, clid, pid, ccid, nodes, has_mpi_trace, has_iprof, mpi_ranks) = &rows[0];
        assert_eq!(*rid, 1);
        assert_eq!(*clid, 1);
        assert_eq!(*pid, 3);
        assert_eq!(*ccid, 1);
        assert_eq!(*nodes, 1);
        assert!(!*has_mpi_trace);
        assert!(!*has_iprof);
        assert_eq!(*mpi_ranks, 8);

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_gromacs_jobdata(pool: sqlx::Pool<MySql>) -> Result<()> {
        // Create CliArgs with project file and other configuration
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/GROMACS/run_64/LMX_summary.376231.0.yml",
            &args,
        )
        .await;

        assert!(
            result.is_ok(),
            "Processing LMX file failed: {:?}",
            result.err()
        );

        // Query the database
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i32, bool, bool, u32)>(
            "SELECT `rid`, `clid`, `pid`, `ccid`, `nodes`, `has_MPItrace`, `has_iprof`, `MPI_ranks` FROM `runs`;"
        )
        .fetch_all(&pool)
        .await?;

        // Assert exactly one row was returned
        assert_eq!(
            rows.len(),
            1,
            "Expected exactly 1 row, but got {}",
            rows.len()
        );

        // Assert the values of the returned row
        let (rid, clid, pid, ccid, nodes, has_mpi_trace, has_iprof, mpi_ranks) = &rows[0];
        assert_eq!(*rid, 1);
        assert_eq!(*clid, 1);
        assert_eq!(*pid, 3);
        assert_eq!(*ccid, 1);
        assert_eq!(*nodes, 1);
        assert!(*has_mpi_trace);
        assert!(*has_iprof);
        assert_eq!(*mpi_ranks, 64);

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_gromacs_jobdata_verbose(pool: sqlx::Pool<MySql>) -> Result<()> {
        // Create CliArgs with verbose = true and dry_run = false
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: true,
            ..Default::default()
        };
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/GROMACS/run_64/LMX_summary.376231.0.yml",
            &args,
        )
        .await;

        assert!(
            result.is_ok(),
            "Processing LMX file failed: {:?}",
            result.err()
        );

        // Query the database
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i32, bool, bool, u32)>(
            "SELECT `rid`, `clid`, `pid`, `ccid`, `nodes`, `has_MPItrace`, `has_iprof`, `MPI_ranks` FROM `runs`;"
        )
        .fetch_all(&pool)
        .await?;

        // Assert exactly one row was returned
        assert_eq!(
            rows.len(),
            1,
            "Expected exactly 1 row, but got {}",
            rows.len()
        );

        // Assert the values of the returned row
        let (rid, clid, pid, ccid, nodes, has_mpi_trace, has_iprof, mpi_ranks) = &rows[0];
        assert_eq!(*rid, 1);
        assert_eq!(*clid, 1);
        assert_eq!(*pid, 3);
        assert_eq!(*ccid, 1);
        assert_eq!(*nodes, 1);
        assert!(*has_mpi_trace);
        assert!(*has_iprof);
        assert_eq!(*mpi_ranks, 64);

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_gromacs_jobdata_dry_run(pool: sqlx::Pool<MySql>) -> Result<()> {
        // Create CliArgs with verbose = false and dry_run = true
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: true,
            verbose: false,
            ..Default::default()
        };
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/GROMACS/run_64/LMX_summary.376231.0.yml",
            &args,
        )
        .await;

        assert!(
            result.is_ok(),
            "Processing LMX file failed: {:?}",
            result.err()
        );

        // Query the database - in dry_run mode, no rows should be inserted
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i32, bool, bool, u32)>(
            "SELECT `rid`, `clid`, `pid`, `ccid`, `nodes`, `has_MPItrace`, `has_iprof`, `MPI_ranks` FROM `runs`;"
        )
        .fetch_all(&pool)
        .await?;

        // Assert no rows were inserted (dry_run mode)
        assert_eq!(
            rows.len(),
            0,
            "Expected 0 rows in dry_run mode, but got {}",
            rows.len()
        );

        Ok(())
    }

    mod test_settings_table {
        use super::*;
        use crate::{
            jobdata::table_settings::import_into_settings_table,
            sqltypes::read_sqltypes,
        };

        /// Test successful import when settings file exists with valid data
        #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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

            // Call import_into_settings_table
            let queries = import_into_settings_table(
                lmx_file.to_str().unwrap(),
                &sqltypes,
                &args,
            ).await?;

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
        #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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

            // Call import_into_settings_table
            let queries = import_into_settings_table(
                lmx_file.to_str().unwrap(),
                &sqltypes,
                &args,
            ).await?;

            // Should return empty vector since settings.yml doesn't exist
            assert!(queries.is_empty(), "Expected empty query list when settings file doesn't exist");

            Ok(())
        }

        /// Test handling when all settings keys match runs table columns (empty value_list case)
        #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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

            // Create a temporary settings file with only keys that exist in runs table
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let test_dir = std::path::Path::new(manifest_dir)
                .join("tests/data/GROMACS/run_64");
            let settings_file = test_dir.join("test_settings_filtered.yml");

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

            let lmx_file = test_dir.join("LMX_summary.376231.0.yml");

            // Call import_into_settings_table
            let queries = import_into_settings_table(
                lmx_file.to_str().unwrap(),
                &sqltypes,
                &args,
            ).await?;

            // Should return empty vector since all keys are filtered
            assert!(queries.is_empty(), "Expected empty query list when all settings keys match runs table columns");

            // Clean up temporary test file
            let _ = std::fs::remove_file(&settings_file);

            Ok(())
        }

        /// Test proper filtering of runs table columns with mixed settings
        #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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

            // Create a temporary settings file with mixed keys
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let test_dir = std::path::Path::new(manifest_dir)
                .join("tests/data/GROMACS/run_64");
            let settings_file = test_dir.join("test_settings_mixed.yml");

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

            let lmx_file = test_dir.join("LMX_summary.376231.0.yml");

            // Call import_into_settings_table
            let queries = import_into_settings_table(
                lmx_file.to_str().unwrap(),
                &sqltypes,
                &args,
            ).await?;

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

            // Clean up temporary test file
            let _ = std::fs::remove_file(&settings_file);

            Ok(())
        }

        /// Test verbose mode outputs message when reading settings
        #[sqlx::test(fixtures("../../tests/fixtures/lmxtest.sql"))]
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

            // Call import_into_settings_table (verbose mode should print message)
            let queries = import_into_settings_table(
                lmx_file.to_str().unwrap(),
                &sqltypes,
                &args,
            ).await?;

            // Should still return queries
            assert!(!queries.is_empty(), "Expected at least one query for settings import in verbose mode");

            Ok(())
        }
    }
}
