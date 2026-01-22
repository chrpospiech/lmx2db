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

        Ok(())
    }
}
