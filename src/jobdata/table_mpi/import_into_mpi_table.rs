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
    use crate::jobdata::table_mpi::import_into_mpi_table;
    use crate::sqltypes::SqlTypeHashMap;
    use anyhow::Result;
    use std::collections::HashMap;

    /// Helper function to create a SqlTypeHashMap with the mpi and mpi_details tables.
    fn create_mpi_sqltypes() -> SqlTypeHashMap {
        let mut sqltypes: SqlTypeHashMap = HashMap::new();

        let mut mpi_columns: HashMap<String, String> = HashMap::new();
        mpi_columns.insert("rid".to_string(), "int(11)".to_string());
        mpi_columns.insert("tid".to_string(), "int(6)".to_string());
        mpi_columns.insert("mid".to_string(), "smallint(8)".to_string());
        mpi_columns.insert("avgbytes".to_string(), "float".to_string());
        mpi_columns.insert("calls".to_string(), "int(11)".to_string());
        mpi_columns.insert("time".to_string(), "float".to_string());
        sqltypes.insert("mpi".to_string(), mpi_columns);

        let mut mpi_details_columns: HashMap<String, String> = HashMap::new();
        mpi_details_columns.insert("rid".to_string(), "int(11)".to_string());
        mpi_details_columns.insert("tid".to_string(), "int(6)".to_string());
        mpi_details_columns.insert("mid".to_string(), "smallint(8)".to_string());
        mpi_details_columns.insert("avgbytes".to_string(), "float".to_string());
        mpi_details_columns.insert("calls".to_string(), "int(11)".to_string());
        mpi_details_columns.insert("time".to_string(), "float".to_string());
        sqltypes.insert("mpi_details".to_string(), mpi_details_columns);

        sqltypes
    }

    /// Test that import_into_mpi_table returns an error when the file name
    /// does not match the expected pattern for extracting a process ID.
    #[test]
    fn test_import_mpi_invalid_file_name() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let result = import_into_mpi_table("invalid_file_name.yml", &sqltypes, &args);
        assert!(
            result.is_err(),
            "Expected error when file name does not match expected pattern"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot extract process ID"),
            "Error message should mention process ID extraction failure"
        );
        Ok(())
    }

    /// Test that import_into_mpi_table returns an empty vector when no MPI
    /// profile files are found for the given LMX summary file.
    #[test]
    fn test_import_mpi_no_mpi_files() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0001/LMX_summary.225250.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;
        assert!(
            queries.is_empty(),
            "Expected empty query list when no MPI profile files exist"
        );
        Ok(())
    }

    /// Test that import_into_mpi_table returns an empty vector when no MPI
    /// profile files are found and verbose mode is enabled.
    #[test]
    fn test_import_mpi_no_mpi_files_verbose() -> Result<()> {
        let args = CliArgs {
            verbose: true,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0001/LMX_summary.225250.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;
        assert!(
            queries.is_empty(),
            "Expected empty query list when no MPI profile files exist in verbose mode"
        );
        Ok(())
    }

    /// Test that import_into_mpi_table returns an empty vector when no MPI
    /// profile files are found and dry_run mode is enabled.
    #[test]
    fn test_import_mpi_no_mpi_files_dry_run() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: true,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0001/LMX_summary.225250.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;
        assert!(
            queries.is_empty(),
            "Expected empty query list when no MPI profile files exist in dry_run mode"
        );
        Ok(())
    }

    /// Test that import_into_mpi_table returns non-empty queries when MPI
    /// profile files are found for the given LMX summary file.
    #[test]
    fn test_import_mpi_with_mpi_files() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0002/LMX_summary.223561.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;
        assert!(
            !queries.is_empty(),
            "Expected non-empty query list when MPI profile files exist"
        );
        Ok(())
    }

    /// Test that import_into_mpi_table generates SQL comments and INSERT
    /// statements in the correct structure.
    #[test]
    fn test_import_mpi_query_structure() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0002/LMX_summary.223561.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;

        // Each MPI profile file produces a comment + INSERT for summary,
        // and a comment + INSERT for details (4 queries per file).
        // There are 5 MPI profile files in NAMD/run_0002.
        assert_eq!(
            queries.len(),
            20,
            "Expected 4 queries per MPI profile file (5 files)"
        );

        // Verify that odd-indexed entries are comments and even-indexed are INSERT statements
        for (i, query) in queries.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    query.starts_with("-- Inserting MPI profile"),
                    "Expected SQL comment at index {}, got: {}",
                    i,
                    query
                );
            } else {
                assert!(
                    query.starts_with("INSERT INTO"),
                    "Expected INSERT statement at index {}, got: {}",
                    i,
                    query
                );
            }
        }
        Ok(())
    }

    /// Test that import_into_mpi_table generates correct INSERT statements
    /// for both the mpi and mpi_details tables.
    #[test]
    fn test_import_mpi_table_names() -> Result<()> {
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let sqltypes = create_mpi_sqltypes();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/NAMD/run_0002/LMX_summary.223561.0.yml");

        let queries = import_into_mpi_table(lmx_file.to_str().unwrap(), &sqltypes, &args)?;

        // For each MPI profile file, the first INSERT should be into the mpi table
        // and the second INSERT should be into the mpi_details table
        assert!(
            queries[1].contains("INSERT INTO mpi"),
            "First INSERT should be into mpi table"
        );
        assert!(
            queries[3].contains("INSERT INTO mpi_details"),
            "Second INSERT should be into mpi_details table"
        );
        Ok(())
    }
}
