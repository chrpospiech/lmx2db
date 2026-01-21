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
    use crate::jobdata::process_lmx_file;
    use crate::jobdata::table_runs::find_file::project_mockup::{
        setup_cliargs_with_project_file_name, setup_tmp_project_directory,
    };
    use crate::sqltypes::{read_sqltypes, SqlTypeHashMap};
    use anyhow::Result;
    use sqlx::MySql;
    use std::fs::remove_dir_all;

    #[sqlx::test(fixtures(
        "../../tests/fixtures/lmxtest.sql",
        "../../tests/fixtures/minimal_data.sql"
    ))]
    pub async fn test_import_namd_jobdata(pool: sqlx::Pool<MySql>) -> Result<()> {
        // Create a temporary project directory for testing
        let temp_dir = setup_tmp_project_directory("tests/data/NAMD")?;
        // Create CliArgs with the specified project file that exists in the temp_dir
        let args = setup_cliargs_with_project_file_name("project.yml")?;
        // Set the LMX_summary file path
        let lmx_summary_pathbuf = temp_dir.join("run_0001/LMX_summary.225250.0.yml");
        // Read SQL types
        let sqltypes: SqlTypeHashMap = read_sqltypes(Some(pool.clone()), &args).await?;

        // Call the process_lmx_file function
        let result = process_lmx_file(
            lmx_summary_pathbuf.to_str().unwrap(),
            &Some(pool.clone()),
            &sqltypes,
            &args,
        )
        .await;

        assert!(
            result.is_ok(),
            "Processing LMX file failed: {:?}",
            result.err()
        );

        // Clean up if necessary
        remove_dir_all(temp_dir)?;

        Ok(())
    }
}
