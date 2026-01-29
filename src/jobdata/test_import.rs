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
pub(crate) mod check_gromacs_data;
#[cfg(test)]
pub(crate) mod check_namd_data;

#[cfg(test)]
mod tests {
    use crate::{
        cmdline::CliArgs,
        jobdata::table_runs::find_file::project_mockup::{
            setup_cliargs_with_project_file_name, test_import_single_lmx_file,
        },
        jobdata::test_import::{
            check_gromacs_data::check_gromacs_data, check_namd_data::check_namd_data,
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

        check_namd_data(&pool).await?;
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

        check_gromacs_data(&pool).await?;

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

        check_gromacs_data(&pool).await?;

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
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM `runs`;")
            .fetch_one(&pool)
            .await?;

        // Assert no rows were inserted (dry_run mode)
        assert_eq!(
            count.0, 0,
            "Expected 0 rows in dry_run mode, but got {}",
            count.0
        );

        Ok(())
    }
}
