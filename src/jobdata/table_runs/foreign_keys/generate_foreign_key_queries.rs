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
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::find_file::project_mockup::{
        setup_cliargs_with_project_file_name, setup_tmp_project_directory,
    };
    use crate::jobdata::table_runs::foreign_keys::generate_foreign_key_queries;
    use crate::jobdata::{read_lmx_summary, LmxSummary};
    use anyhow::Result;
    use std::fs::remove_dir_all;

    // Test generating foreign key queries when the project file is missing
    // We test with pool = None to avoid actual DB operations
    #[tokio::test]
    pub async fn test_generate_foreign_key_queries_with_simple_namd_data() -> Result<()> {
        // Create a temporary project file for testing
        let temp_dir = setup_tmp_project_directory("tests/data/NAMD")?;
        let project_file = temp_dir.join("project.yml");
        let args = setup_cliargs_with_project_file_name(project_file.to_str().unwrap())?;

        // Set the LMX_summary file path and read its contents
        let lmx_summary_pathbuf = temp_dir.join("run_0001/LMX_summary.225250.0.yml");
        let lmx_summary: LmxSummary = read_lmx_summary(lmx_summary_pathbuf.to_str().unwrap())
            .expect("Failed to read LMX summary");

        // Call the generate_foreign_key_queries function
        let sql_queries = generate_foreign_key_queries(
            lmx_summary_pathbuf.to_str().unwrap(),
            &None,
            &lmx_summary,
            &args,
        )
        .await;
        assert!(sql_queries.is_ok(), "{}", sql_queries.as_ref().unwrap_err());
        let sql_queries = sql_queries.unwrap();
        assert_eq!(sql_queries.len(), 5);
        assert_eq!(sql_queries[0], "SET @clid = cluster_id('Lenox', 0);");
        assert_eq!(
            sql_queries[1],
            "SET @pid = person_id_for_uid('xcpospiech', cluster_id('Lenox', 0));"
        );
        assert_eq!(
            sql_queries[2],
            "SET @ccid = customer_case_id('4paper_2025', 'NAMD', '3.0.2', 'STMV', 0);"
        );
        assert_eq!(
            sql_queries[3],
            "SET @fsid = filesystem_id('GPFS', '/gpfs/gpfs_de6000', 8192);"
        );
        assert_eq!(
            sql_queries[4],
            "CALL drop_run_by_user_start_date(@pid, 1764250902, 199871237);"
        );

        // Clean up the temporary project file and directory
        remove_dir_all(temp_dir)?;
        Ok(())
    }

    // Test generating foreign key queries when the project file is present
    // We test with pool = None to allow actual DB operations
    #[tokio::test]
    pub async fn test_do_import_with_simple_namd_data() -> Result<()> {
        // Create a temporary project file for testing
        let temp_dir = setup_tmp_project_directory("tests/data/NAMD")?;
        let project_file = temp_dir.join("project.yml");
        // Create CliArgs with the specified project file
        let args = CliArgs {
            project_file: project_file.to_str().unwrap().to_string(),
            verbose: false,
            dry_run: false,
            do_import: true,
            ..Default::default()
        };

        // Set the LMX_summary file path and read its contents
        let lmx_summary_pathbuf = temp_dir.join("run_0001/LMX_summary.225250.0.yml");
        let lmx_summary: LmxSummary = read_lmx_summary(lmx_summary_pathbuf.to_str().unwrap())
            .expect("Failed to read LMX summary");

        // Call the generate_foreign_key_queries function
        let sql_queries = generate_foreign_key_queries(
            lmx_summary_pathbuf.to_str().unwrap(),
            &None,
            &lmx_summary,
            &args,
        )
        .await;
        assert!(sql_queries.is_ok(), "{}", sql_queries.as_ref().unwrap_err());
        let sql_queries = sql_queries.unwrap();
        assert_eq!(sql_queries.len(), 5);
        assert_eq!(sql_queries[0], "SET @clid = cluster_id('Lenox', 1);");
        assert_eq!(
            sql_queries[1],
            "SET @pid = person_id_for_uid('xcpospiech', cluster_id('Lenox', 1));"
        );
        assert_eq!(
            sql_queries[2],
            "SET @ccid = customer_case_id('4paper_2025', 'NAMD', '3.0.2', 'STMV', 1);"
        );
        assert_eq!(
            sql_queries[3],
            "SET @fsid = filesystem_id('GPFS', '/gpfs/gpfs_de6000', 8192);"
        );
        assert_eq!(
            sql_queries[4],
            "CALL drop_run_by_user_start_date(@pid, 1764250902, 199871237);"
        );

        // Clean up the temporary project file and directory
        remove_dir_all(temp_dir)?;
        Ok(())
    }
}
