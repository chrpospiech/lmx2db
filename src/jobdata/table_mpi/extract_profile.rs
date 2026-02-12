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

    use crate::jobdata::table_mpi::extract_mpi_data_from_mpi_profile;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;

    /// Helper function to create a default LmxSummary for testing purposes.
    /// The default summary can be modified to include specific data needed for testing.
    /// Returns a default LmxSummary instance.
    fn create_default_lmx_summary(with_mpi_rank: bool) -> Result<LmxSummary> {
        let mpi_key = if with_mpi_rank {
            "my_MPI_rank".to_string()
        } else {
            "something_else".to_string()
        };
        let template = r#"
base_data:
    something_else: n/a
    {}: 0
MPI_rank_summary:
    MPI_Send: [2, 66.0, 5.006790e-06]
wrong_summary:
    MPI_send: [2, 66.0, 5.006790e-06, 15]
empty_summary:
MPI_rank_details:
    MPI_Send:
        - [1, 24.0, 2.384186e-06]
        - [1, 108.0, 2.622604e-06]
        "#;
        let yaml_str = template.replace("{}", &mpi_key);
        let summary: LmxSummary = serde_yaml::from_str(&yaml_str)?;
        Ok(summary)
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_empty_input() -> Result<()> {
        let summary = LmxSummary::default();
        let result = extract_mpi_data_from_mpi_profile(&summary, "");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_no_mpi_rank() -> Result<()> {
        let summary = create_default_lmx_summary(false)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "invalid_key");
        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank key is missing"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("my_MPI_rank key not found"),
            "Error message should mention missing my_MPI_rank key"
        );
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_missing_section() -> Result<()> {
        let summary = create_default_lmx_summary(true)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "invalid_key");
        assert!(
            result.is_err(),
            "Expected error when section key is invalid"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("key not found in LMX summary"),
            "Error message should mention missing section key"
        );
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_empty_section() -> Result<()> {
        let summary = create_default_lmx_summary(true)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "empty_summary");
        assert!(
            result.is_err(),
            "Expected error for section key without data structure"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No MPI profiles found in section"),
            "Error message should mention missing data structure"
        );
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_invalid_data() -> Result<()> {
        let summary = create_default_lmx_summary(true)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "wrong_summary");
        assert!(
            result.is_err(),
            "Expected error for section key with invalid data structure"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected 3 values for MPI profile data"),
            "Error message should mention expected number of values"
        );
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_returns_single_line() -> Result<()> {
        let summary = create_default_lmx_summary(true)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "MPI_rank_summary");
        assert!(
            result.is_ok(),
            "Expected successful extraction of MPI profile data"
        );
        if let Ok(data) = result {
            assert!(data.len() == 1, "Expected one row of data");
            assert!(data[0].len() == 6, "Expected 6 values in the output row");
        }
        Ok(())
    }

    #[test]
    fn test_extract_mpi_data_from_mpi_profile_multi_line() -> Result<()> {
        let summary = create_default_lmx_summary(true)?;
        let result = extract_mpi_data_from_mpi_profile(&summary, "MPI_rank_details");
        assert!(
            result.is_ok(),
            "Expected successful extraction of MPI profile data"
        );
        if let Ok(data) = result {
            assert!(data.len() == 2, "Expected two rows of data");
            assert!(data[0].len() == 6, "Expected 6 values in the output row");
        }
        Ok(())
    }
}
