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
    use crate::jobdata::mpi_ranks::extract_mpi_rank;
    use crate::jobdata::LmxSummary;
    use std::collections::HashMap;

    /// Test error handling when base_data key is missing
    #[test]
    fn test_extract_mpi_rank_missing_base_data() {
        let lmx_summary: LmxSummary = HashMap::new();

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_err(), "Expected error when base_data is missing");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("base_data key not found"),
            "Error message should mention missing base_data"
        );
    }

    /// Test error handling when my_MPI_rank key is missing
    #[test]
    fn test_extract_mpi_rank_missing_mpi_rank_key() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let base_data: HashMap<String, serde_yaml::Value> = HashMap::new();
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

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
    }

    /// Test handling when my_MPI_rank value is a positive number (should succeed)
    #[test]
    fn test_extract_mpi_rank_positive_number_value() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a number - this should now work
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number(42.into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(
            result.is_ok(),
            "Expected success when my_MPI_rank is a positive number"
        );
        assert_eq!(result.unwrap(), 42, "Expected MPI rank to be 42");
    }

    /// Test error handling when my_MPI_rank value cannot be parsed as u64
    #[test]
    fn test_extract_mpi_rank_invalid_parse() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a string that cannot be parsed as u64
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("not_a_number".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank cannot be parsed as u64"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse my_MPI_rank"),
            "Error message should mention parse failure"
        );
    }

    /// Test error handling when my_MPI_rank is a negative number
    #[test]
    fn test_extract_mpi_rank_negative_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a negative number (as i64)
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number((-1).into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank is negative"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("negative"),
            "Error message should mention negative value"
        );
    }

    /// Test error handling when my_MPI_rank has invalid characters
    #[test]
    fn test_extract_mpi_rank_invalid_characters() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("12abc".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank has invalid characters"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse my_MPI_rank"),
            "Error message should mention parse failure"
        );
    }

    /// Test error handling when my_MPI_rank is an empty string
    #[test]
    fn test_extract_mpi_rank_empty_string() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_err(), "Expected error when my_MPI_rank is empty");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse my_MPI_rank"),
            "Error message should mention parse failure"
        );
    }

    /// Test error handling when my_MPI_rank is neither a number nor a string
    #[test]
    fn test_extract_mpi_rank_invalid_type() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a boolean (neither number nor string)
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Bool(true),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank is neither number nor string"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("neither a number nor a string"),
            "Error message should mention invalid type"
        );
    }
}
