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

    /// Test error handling for invalid value types
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_environ_invalid_value_type(pool: sqlx::Pool<MySql>) -> Result<()> {
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

        // Create an LMX summary with invalid environ value (number instead of string/sequence)
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut environ_section = HashMap::new();

        environ_section.insert(
            "INVALID_VAR".to_string(),
            serde_yaml::Value::Number(42.into()),
        );

        lmx_summary.insert("environ".to_string(), environ_section);

        // Call import_into_environ_table - should return an error
        let result = import_into_environ_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when environ value is not a String or Sequence"
        );

        Ok(())
    }

    /// Test error handling for non-string values in sequence
    #[sqlx::test(fixtures("../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_import_environ_invalid_sequence_element(
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

        // Create an LMX summary with sequence containing non-string values
        let mut lmx_summary: crate::jobdata::LmxSummary = HashMap::new();
        let mut environ_section = HashMap::new();

        environ_section.insert(
            "BAD_SEQUENCE".to_string(),
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("valid".to_string()),
                serde_yaml::Value::Number(123.into()), // Invalid: number in sequence
            ]),
        );

        lmx_summary.insert("environ".to_string(), environ_section);

        // Call import_into_environ_table - should return an error
        let result = import_into_environ_table(&lmx_summary, &sqltypes, &args);

        assert!(
            result.is_err(),
            "Expected error when sequence contains non-string values"
        );

        Ok(())
    }
}
