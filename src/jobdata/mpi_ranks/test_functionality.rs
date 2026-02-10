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

    /// Test successful extraction of MPI rank with value 0
    #[test]
    fn test_extract_mpi_rank_zero() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("0".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 0, "Expected MPI rank to be 0");
    }

    /// Test successful extraction of MPI rank with a small positive value
    #[test]
    fn test_extract_mpi_rank_small_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("42".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 42, "Expected MPI rank to be 42");
    }

    /// Test successful extraction of MPI rank with a large value
    #[test]
    fn test_extract_mpi_rank_large_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("999999".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 999999, "Expected MPI rank to be 999999");
    }

    /// Test successful extraction of MPI rank with maximum u64 value
    #[test]
    fn test_extract_mpi_rank_max_u64() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        let max_u64 = u64::MAX.to_string();
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String(max_u64.clone()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(
            result.unwrap(),
            u64::MAX,
            "Expected MPI rank to be u64::MAX"
        );
    }

    /// Test successful extraction with leading zeros
    #[test]
    fn test_extract_mpi_rank_leading_zeros() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("00042".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(
            result.unwrap(),
            42,
            "Expected MPI rank to be 42 (leading zeros ignored)"
        );
    }

    /// Test successful extraction with typical MPI rank value
    #[test]
    fn test_extract_mpi_rank_typical_value() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("127".to_string()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 127, "Expected MPI rank to be 127");
    }

    /// Test that base_data can contain other keys
    #[test]
    fn test_extract_mpi_rank_with_other_keys() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Add other keys to base_data
        base_data.insert(
            "other_key".to_string(),
            serde_yaml::Value::String("other_value".to_string()),
        );
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::String("5".to_string()),
        );
        base_data.insert(
            "another_key".to_string(),
            serde_yaml::Value::Number(123.into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_mpi_rank(&lmx_summary);

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(
            result.unwrap(),
            5,
            "Expected MPI rank to be 5 regardless of other keys"
        );
    }
}
