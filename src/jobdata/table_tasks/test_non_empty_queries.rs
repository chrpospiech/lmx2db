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
        cmdline::CliArgs, jobdata::table_tasks::import_into_tasks_table,
        sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;

    /// Test successful import with real GROMACS data
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_with_valid_gromacs_data(
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

        // Use the GROMACS test data
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");
        let lmx_summary: crate::jobdata::LmxSummary =
            serde_yaml::from_str(&std::fs::read_to_string(&lmx_file)?)?;

        // Call import_into_tasks_table
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should return a comment line, SET statement, and exactly one INSERT query
        assert_eq!(
            queries.len(),
            3,
            "Expected a comment line, SET statement, and one INSERT query for tasks import"
        );

        // Verify the comment
        assert!(
            queries[0].contains("-- Inserting into tasks table"),
            "First query should be a comment"
        );

        // Verify the SET statement
        assert!(
            queries[1].contains("SET @cl_name = cluster_name(@clid)"),
            "Second query should set @cl_name"
        );

        // Verify the INSERT query
        let query = &queries[2];
        assert!(
            query.contains("INSERT"),
            "Query should be an INSERT statement"
        );
        assert!(
            query.contains("tasks"),
            "Query should insert into tasks table"
        );

        // Verify key columns are present
        assert!(query.contains("rid"), "Query should include rid column");
        assert!(query.contains("tid"), "Query should include tid column");
        assert!(query.contains("lid"), "Query should include lid column");
        assert!(
            query.contains("affinity"),
            "Query should include affinity column"
        );
        assert!(
            query.contains("elapsed"),
            "Query should include elapsed column"
        );
        assert!(
            query.contains("usertime"),
            "Query should include usertime column"
        );
        assert!(
            query.contains("systime"),
            "Query should include systime column"
        );
        assert!(
            query.contains("memory"),
            "Query should include memory column"
        );
        assert!(
            query.contains("vmemory"),
            "Query should include vmemory column"
        );

        // Optional sections that should be present in GROMACS data
        assert!(query.contains("comm"), "Query should include comm column");
        assert!(
            query.contains("mpiio"),
            "Query should include mpiio column"
        );
        assert!(
            query.contains("loadimb"),
            "Query should include loadimb column"
        );

        // Verify that location_id() function is used
        assert!(
            query.contains("location_id("),
            "Query should use location_id() function"
        );

        Ok(())
    }

    /// Test successful import with minimal required sections
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_minimal_sections(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create a minimal LMX summary with only required sections
        let mut lmx_summary: std::collections::HashMap<String, std::collections::HashMap<String, serde_yaml::Value>> =
            std::collections::HashMap::new();

        let mut cpu_affinity = std::collections::HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        cpu_affinity.insert(
            "1".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0002".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = std::collections::HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.5.into()),
                serde_yaml::Value::Number(80.2.into()),
                serde_yaml::Value::Number(10.3.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.5.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call import_into_tasks_table
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should return queries
        assert_eq!(
            queries.len(),
            3,
            "Expected a comment, SET statement, and one query for minimal tasks import"
        );

        let query = &queries[2];

        // Verify mandatory columns are present
        assert!(query.contains("rid"), "Query should include rid column");
        assert!(query.contains("tid"), "Query should include tid column");
        assert!(query.contains("lid"), "Query should include lid column");
        assert!(
            query.contains("affinity"),
            "Query should include affinity column"
        );
        assert!(
            query.contains("elapsed"),
            "Query should include elapsed column"
        );

        // Optional sections should NOT be present
        assert!(
            !query.contains("comm"),
            "Query should not include comm column when communication_times is missing"
        );
        assert!(
            !query.contains("mpiio"),
            "Query should not include mpiio column when communication_times is missing"
        );
        assert!(
            !query.contains("loadimb"),
            "Query should not include loadimb column when load_imbalance_times is missing"
        );

        Ok(())
    }

    /// Test verbose mode outputs message for missing optional sections
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_verbose_missing_optional(
        pool: sqlx::Pool<MySql>,
    ) -> Result<()> {
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

        // Create a minimal LMX summary without optional sections
        let mut lmx_summary: std::collections::HashMap<String, std::collections::HashMap<String, serde_yaml::Value>> =
            std::collections::HashMap::new();

        let mut cpu_affinity = std::collections::HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        cpu_affinity.insert(
            "1".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0002".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = std::collections::HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.0.into()),
                serde_yaml::Value::Number(80.0.into()),
                serde_yaml::Value::Number(10.0.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.0.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call import_into_tasks_table in verbose mode
        // Should print messages about missing sections (not testing stdout here)
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should still return queries
        assert!(
            !queries.is_empty(),
            "Expected at least one query for tasks import in verbose mode"
        );

        Ok(())
    }

    /// Test backward compatibility with 'affinity' section name
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_backward_compat_affinity(
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

        // Create LMX summary with 'affinity' instead of 'CPU_affinity'
        let mut lmx_summary: std::collections::HashMap<String, std::collections::HashMap<String, serde_yaml::Value>> =
            std::collections::HashMap::new();

        let mut affinity = std::collections::HashMap::new();
        affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        affinity.insert(
            "1".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node1.example.com".to_string()),
                serde_yaml::Value::String("0002".to_string()),
            ]),
        );
        lmx_summary.insert("affinity".to_string(), affinity);

        let mut rank_summary = std::collections::HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.0.into()),
                serde_yaml::Value::Number(80.0.into()),
                serde_yaml::Value::Number(10.0.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.0.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call import_into_tasks_table with 'affinity' section
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should successfully process with backward compatible section name
        assert_eq!(
            queries.len(),
            3,
            "Expected queries with backward compatible affinity section"
        );

        Ok(())
    }

    /// Test with all optional sections present
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_all_optional_sections(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create LMX summary with all sections
        let mut lmx_summary: std::collections::HashMap<String, std::collections::HashMap<String, serde_yaml::Value>> =
            std::collections::HashMap::new();

        let mut cpu_affinity = std::collections::HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        cpu_affinity.insert(
            "1".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0002".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = std::collections::HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.0.into()),
                serde_yaml::Value::Number(80.0.into()),
                serde_yaml::Value::Number(10.0.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.0.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        let mut comm_times = std::collections::HashMap::new();
        comm_times.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(50.0.into()),
                serde_yaml::Value::Number(5.0.into()),
                serde_yaml::Value::Number(2.0.into()),
                serde_yaml::Value::Number(1.0.into()),
            ]),
        );
        lmx_summary.insert("communication_times".to_string(), comm_times);

        let mut loadimb_times = std::collections::HashMap::new();
        loadimb_times.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(3.5.into()),
                serde_yaml::Value::Number(1.2.into()),
                serde_yaml::Value::Number(0.8.into()),
            ]),
        );
        lmx_summary.insert("load_imbalance_times".to_string(), loadimb_times);

        // Call import_into_tasks_table
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should return queries with all columns
        assert_eq!(
            queries.len(),
            3,
            "Expected queries with all optional sections"
        );

        let query = &queries[2];

        // Verify all columns are present
        assert!(query.contains("comm"), "Query should include comm column");
        assert!(
            query.contains("mpiio"),
            "Query should include mpiio column"
        );
        assert!(
            query.contains("loadimb"),
            "Query should include loadimb column"
        );

        Ok(())
    }

    /// Test dry_run mode outputs message for missing optional sections
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_dry_run_missing_optional(
        pool: sqlx::Pool<MySql>,
    ) -> Result<()> {
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

        // Create a minimal LMX summary without optional sections
        let mut lmx_summary: std::collections::HashMap<String, std::collections::HashMap<String, serde_yaml::Value>> =
            std::collections::HashMap::new();

        let mut cpu_affinity = std::collections::HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        cpu_affinity.insert(
            "1".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0002".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = std::collections::HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.0.into()),
                serde_yaml::Value::Number(80.0.into()),
                serde_yaml::Value::Number(10.0.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.0.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call import_into_tasks_table in dry_run mode
        // Should print messages about missing sections (not testing stdout here)
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should still return queries
        assert!(
            !queries.is_empty(),
            "Expected at least one query for tasks import in dry_run mode"
        );

        Ok(())
    }
}
