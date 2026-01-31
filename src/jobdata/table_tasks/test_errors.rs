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
        cmdline::CliArgs, jobdata::table_tasks::import_into_tasks_table, sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;
    use std::collections::HashMap;

    /// Test error handling when CPU_affinity section is missing
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_missing_affinity_section(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create an LMX summary without CPU_affinity section
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut rank_summary = HashMap::new();
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

        // Call should fail due to missing CPU_affinity section
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when CPU_affinity section is missing"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing mandatory section 'CPU_affinity'"),
            "Error message should mention missing CPU_affinity section"
        );

        Ok(())
    }

    /// Test error handling when rank_summary section is missing
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_missing_rank_summary_section(
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

        // Create an LMX summary with CPU_affinity but without rank_summary
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        // Call should fail due to missing rank_summary section
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when rank_summary section is missing"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing mandatory section 'rank_summary'"),
            "Error message should mention missing rank_summary section"
        );

        Ok(())
    }

    /// Test error handling when CPU_affinity value is not a sequence
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_invalid_affinity_type(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create an LMX summary with invalid CPU_affinity (not a sequence)
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::String("invalid".to_string()),
        );
        cpu_affinity.insert(
            "1".to_string(),
            serde_yaml::Value::String("invalid".to_string()),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = HashMap::new();
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

        // Call should fail due to invalid affinity type
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when affinity is not a sequence"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected a sequence (array)"),
            "Error message should mention expected sequence"
        );

        Ok(())
    }

    /// Test error handling when rank_summary value is not a sequence
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_invalid_rank_summary_type(
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

        // Create an LMX summary with invalid rank_summary (not a sequence)
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
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
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::String("invalid".to_string()),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call should fail due to invalid rank_summary type
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when rank_summary is not a sequence"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected a sequence (array)"),
            "Error message should mention expected sequence"
        );

        Ok(())
    }

    /// Test error handling when rank_summary contains non-float values
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_invalid_rank_summary_elements(
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

        // Create an LMX summary with rank_summary containing non-float elements
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
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
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = HashMap::new();
        rank_summary.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(100.0.into()),
                serde_yaml::Value::String("not_a_float".to_string()),
                serde_yaml::Value::Number(10.0.into()),
                serde_yaml::Value::Number(200.0.into()),
                serde_yaml::Value::Number(300.0.into()),
            ]),
        );
        lmx_summary.insert("rank_summary".to_string(), rank_summary);

        // Call should fail due to non-float value in rank_summary
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when rank_summary contains non-float values"
        );
        assert!(
            result.unwrap_err().to_string().contains("Expected a float"),
            "Error message should mention expected float"
        );

        Ok(())
    }

    /// Test error handling when communication_times has insufficient elements
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_insufficient_communication_times(
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

        // Create an LMX summary with communication_times having fewer than 3 elements
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
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
                serde_yaml::Value::String("0001".to_string()),
            ]),
        );
        lmx_summary.insert("CPU_affinity".to_string(), cpu_affinity);

        let mut rank_summary = HashMap::new();
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

        let mut comm_times = HashMap::new();
        // Only 2 elements instead of 3
        comm_times.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Number(50.0.into()),
                serde_yaml::Value::Number(5.0.into()),
            ]),
        );
        lmx_summary.insert("communication_times".to_string(), comm_times);

        // Call should fail due to insufficient communication_times elements
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when communication_times has fewer than 3 elements"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected at least 3 communication_times entries"),
            "Error message should mention expected 3 entries"
        );

        Ok(())
    }

    /// Test error handling when CPU_affinity contains non-hexadecimal characters
    /// Note: This test is currently ignored because hexadecimal validation is not yet implemented.
    /// Remove the #[ignore] attribute once validation is added to the import_into_tasks_table function.
    #[ignore]
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_tasks_invalid_hexadecimal_in_affinity(
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

        // Create an LMX summary with CPU_affinity containing invalid hex characters
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut cpu_affinity = HashMap::new();
        cpu_affinity.insert(
            "0".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("node0.example.com".to_string()),
                // Invalid: contains 'k' which is not a hexadecimal digit
                serde_yaml::Value::String("00010k".to_string()),
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

        let mut rank_summary = HashMap::new();
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

        // Call should fail due to non-hexadecimal characters in CPU_affinity
        let result = import_into_tasks_table(&lmx_summary, &sqltypes, &args);

        // When hexadecimal validation is implemented, this should fail
        assert!(
            result.is_err(),
            "Expected error when CPU_affinity contains non-hexadecimal characters"
        );
        
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("hexadecimal") || error_msg.contains("invalid character"),
            "Error message should mention hexadecimal validation: {}",
            error_msg
        );

        Ok(())
    }
}
