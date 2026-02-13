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
    use crate::jobdata::base_data::extract_base_data_key;
    use crate::jobdata::LmxSummary;
    use std::collections::HashMap;

    /// Test error handling when base_data key is missing
    #[test]
    fn test_extract_base_data_key_missing_base_data() {
        let lmx_summary: LmxSummary = HashMap::new();

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

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
    fn test_extract_base_data_key_missing_mpi_rank_key() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let base_data: HashMap<String, serde_yaml::Value> = HashMap::new();
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

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

    /// Test error handling when my_MPI_rank value is not a numeric type
    #[test]
    fn test_extract_base_data_key_non_numeric_type() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a string (non-numeric type)
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("not_a_number".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank is not a numeric type"
        );
        assert!(
            result.unwrap_err().to_string().contains(
                "my_MPI_rank value in base_data is not a number that can be converted to u64"
            ),
            "Error message should mention non-numeric type"
        );
    }

    /// Test error handling when my_MPI_rank is a negative number
    #[test]
    fn test_extract_base_data_key_negative_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Insert a negative number (as i64)
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number((-1).into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

        assert!(
            result.is_err(),
            "Expected error when my_MPI_rank is negative"
        );
        assert!(
            result.unwrap_err().to_string().contains("negative"),
            "Error message should mention negative value"
        );
    }
}
