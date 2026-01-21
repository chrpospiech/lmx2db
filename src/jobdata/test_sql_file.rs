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
mod test {
    use crate::cmdline::CliArgs;
    use crate::jobdata::process_sql_queries;
    use anyhow::Result;
    use sqlx::MySql;
    use std::fs::OpenOptions;
    use std::io::Read;
    use tempfile::NamedTempFile;

    /// Helper function reading a SQL file and verifying its content against expected lines.
    /// Each expected line must be present in the file in the given order.
    ///
    /// # Arguments
    /// * `file_name` - Path to the SQL file to read
    /// * `expected_lines` - Vector of expected lines to verify in the file
    ///
    /// # Returns
    /// Returns `Ok(())` if all expected lines are found in order,
    /// or an error if any line is missing or out of order.
    ///
    /// # Errors
    /// Returns an `anyhow::Error` if file reading fails or verification fails.
    ///
    pub fn test_process_sql_file(file_name: &str, expected_lines: Vec<&str>) -> Result<()> {
        let mut file = OpenOptions::new().read(true).open(file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let expected_content = expected_lines.join("\n") + "\n";
        assert_eq!(content, expected_content);
        Ok(())
    }

    #[tokio::test]
    pub async fn test_file_with_transaction() -> Result<()> {
        // Create a temporary file for test output
        // Keep temp_file in scope to prevent automatic deletion until test completes
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_string_lossy().to_string();

        // Setup test arguments and mock database pool
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            sql_file: temp_path.clone(),
            ..Default::default()
        };
        let pool: Option<sqlx::Pool<MySql>> = None;
        let query_list = vec![
            "INSERT INTO test_table (col1, col2) VALUES ('val1', 'val2');".to_string(),
            "UPDATE test_table SET col2 = 'val3' WHERE col1 = 'val1';".to_string(),
        ];
        // Call the function to test
        process_sql_queries(query_list, &pool, &args).await?;
        // Read and verify the output file
        test_process_sql_file(
            &args.sql_file,
            vec![
                "START TRANSACTION;",
                "INSERT INTO test_table (col1, col2) VALUES ('val1', 'val2');",
                "UPDATE test_table SET col2 = 'val3' WHERE col1 = 'val1';",
                "COMMIT;",
            ],
        )?;
        // Temporary file will be automatically cleaned up when temp_file goes out of scope
        Ok(())
    }
}
