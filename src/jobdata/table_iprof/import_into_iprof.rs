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
        cmdline::CliArgs, jobdata::table_iprof::import_into_iprof_table, sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    /// Test that the function returns empty Vec when no itimer profile files are found
    #[test]
    fn test_import_iprof_no_files() -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create a temporary directory without any itimer profile files
        let temp_dir = TempDir::new()?;
        let temp_file = temp_dir.path().join("LMX_summary.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&temp_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an empty sqltypes map
        let sqltypes: HashMap<String, HashMap<String, String>> = HashMap::new();

        // Call import_into_iprof_table with a file that has no corresponding itimer files
        let queries = import_into_iprof_table(temp_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return empty vector when no itimer files are found
        assert!(
            queries.is_empty(),
            "Expected empty query list when no itimer profile files found"
        );

        Ok(())
    }

    /// Test that the function skips files with total_ticks == 0
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_iprof_zero_total_ticks(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a temporary directory with an itimer profile file that has zero total ticks
        let temp_dir = TempDir::new()?;
        let summary_file = temp_dir.path().join("LMX_summary.123456.0.yml");
        let iprof_file = temp_dir.path().join("LMX_itimer_profile.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&summary_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an itimer profile file with zero total ticks
        let iprof_content = r#"base_data:
  my_MPI_rank: 0
  itimer_ticks_total: 0
library_names:
  lib1:
  - "libtest.so"
library_histogram:
  lib1: [100, 50.0]
"#;
        fs::write(&iprof_file, iprof_content)?;

        // Call import_into_iprof_table
        let queries = import_into_iprof_table(summary_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return empty vector when total_ticks is zero
        assert!(
            queries.is_empty(),
            "Expected empty query list when total_ticks is zero"
        );

        Ok(())
    }

    /// Test that the function handles missing library_histogram section gracefully
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_iprof_missing_library_histogram(
        pool: sqlx::Pool<MySql>,
    ) -> Result<()> {
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

        // Create a temporary directory with an itimer profile file missing library_histogram
        let temp_dir = TempDir::new()?;
        let summary_file = temp_dir.path().join("LMX_summary.123456.0.yml");
        let iprof_file = temp_dir.path().join("LMX_itimer_profile.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&summary_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an itimer profile file without library_histogram section
        let iprof_content = r#"base_data:
  my_MPI_rank: 0
  itimer_ticks_total: 100
library_names:
  lib1:
  - "libtest.so"
"#;
        fs::write(&iprof_file, iprof_content)?;

        // Call import_into_iprof_table
        let queries = import_into_iprof_table(summary_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return only the total ticks query, not library histogram queries
        // One query for the total ticks
        assert_eq!(
            queries.len(),
            1,
            "Expected only 1 query when library_histogram is missing"
        );

        // Verify it's the total ticks query
        let query = &queries[0];
        assert!(
            query.contains("__total__"),
            "Query should contain total ticks entry"
        );

        Ok(())
    }

    /// Test that the function handles missing flat_profile section gracefully
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_iprof_missing_flat_profile(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a temporary directory with an itimer profile file missing flat_profile
        let temp_dir = TempDir::new()?;
        let summary_file = temp_dir.path().join("LMX_summary.123456.0.yml");
        let iprof_file = temp_dir.path().join("LMX_itimer_profile.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&summary_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an itimer profile file with library_histogram but no flat_profile
        let iprof_content = r#"base_data:
  my_MPI_rank: 0
  itimer_ticks_total: 100
library_names:
  lib1:
  - "libtest.so"
library_histogram:
  lib1: [80, 80.0]
"#;
        fs::write(&iprof_file, iprof_content)?;

        // Call import_into_iprof_table
        let queries = import_into_iprof_table(summary_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return total ticks query and library histogram query, but not flat_profile queries
        // Two queries: total ticks + library histogram
        assert_eq!(
            queries.len(),
            2,
            "Expected 2 queries when flat_profile is missing"
        );

        // Verify the first query is for total ticks
        assert!(
            queries[0].contains("__total__"),
            "First query should contain total ticks entry"
        );

        // Verify the second query is for library histogram
        assert!(
            queries[1].contains("libtest.so"),
            "Second query should contain library name"
        );

        Ok(())
    }

    /// Test with empty library_histogram section
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_iprof_empty_library_histogram(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a temporary directory with an itimer profile file with empty library_histogram
        let temp_dir = TempDir::new()?;
        let summary_file = temp_dir.path().join("LMX_summary.123456.0.yml");
        let iprof_file = temp_dir.path().join("LMX_itimer_profile.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&summary_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an itimer profile file with empty library_histogram
        let iprof_content = r#"base_data:
  my_MPI_rank: 0
  itimer_ticks_total: 100
library_names: {}
library_histogram: {}
"#;
        fs::write(&iprof_file, iprof_content)?;

        // Call import_into_iprof_table
        let queries = import_into_iprof_table(summary_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return only the total ticks query
        assert_eq!(
            queries.len(),
            1,
            "Expected only 1 query when library_histogram is empty"
        );

        Ok(())
    }

    /// Test with empty flat_profile section
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_iprof_empty_flat_profile(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a temporary directory with an itimer profile file with empty flat_profile
        let temp_dir = TempDir::new()?;
        let summary_file = temp_dir.path().join("LMX_summary.123456.0.yml");
        let iprof_file = temp_dir.path().join("LMX_itimer_profile.123456.0.yml");

        // Create a minimal LMX summary file
        fs::write(&summary_file, "base_data:\n  my_MPI_rank: 0\n")?;

        // Create an itimer profile file with library_histogram but empty flat_profile
        let iprof_content = r#"base_data:
  my_MPI_rank: 0
  itimer_ticks_total: 100
library_names:
  lib1:
  - "libtest.so"
library_histogram:
  lib1: [80, 80.0]
flat_profile: {}
"#;
        fs::write(&iprof_file, iprof_content)?;

        // Call import_into_iprof_table
        let queries = import_into_iprof_table(summary_file.to_str().unwrap(), &sqltypes, &args)?;

        // Should return total ticks and library histogram queries
        assert_eq!(
            queries.len(),
            2,
            "Expected 2 queries when flat_profile is empty"
        );

        Ok(())
    }
}
