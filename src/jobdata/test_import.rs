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
}
