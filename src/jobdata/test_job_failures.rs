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
    use crate::jobdata::table_runs::find_file::project_mockup::{
        setup_cliargs_with_project_file_name, test_import_single_lmx_file,
    };
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures(
        "../../tests/fixtures/tables.sql",
        "../../tests/fixtures/functs4test.sql"
    ))]
    async fn test_missing_project_file_with_simple_namd_data(pool: Pool<MySql>) -> Result<()> {
        // Create CliArgs with the specified project file that does not exist
        let args = setup_cliargs_with_project_file_name("not_there.yml")?;
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/NAMD/run_0001/LMX_summary.225250.0.yml",
            &args,
        )
        .await;
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(
            error_message.contains("Required project file 'not_there.yml' not found"),
            "Unexpected error message: {}",
            error_message
        );
        Ok(())
    }

    #[sqlx::test(fixtures(
        "../../tests/fixtures/tables.sql",
        "../../tests/fixtures/functs4test.sql"
    ))]
    async fn test_cluster_id_with_simple_namd_data(pool: Pool<MySql>) -> Result<()> {
        // Create CliArgs with the specified project file that exists for the provided data
        let args = setup_cliargs_with_project_file_name("project.yml")?;
        // Call the test_import_single_lmx_file
        let result = test_import_single_lmx_file(
            &pool,
            None,
            "tests/data/NAMD/run_0001/LMX_summary.225250.0.yml",
            &args,
        )
        .await;
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(
            error_message
                .contains("Foreign key validation failed: query returned no result or NULL."),
            "Unexpected error message: {}",
            error_message
        );
        Ok(())
    }
}
