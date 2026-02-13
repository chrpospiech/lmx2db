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

    /// Test successful extraction of MPI rank with value 0 (as number)
    #[test]
    fn test_extract_base_data_key_zero_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number(0.into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 0, "Expected MPI rank to be 0");
    }

    /// Test successful extraction of MPI rank with a small positive value (as number)
    #[test]
    fn test_extract_base_data_key_small_number_u64() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number(42.into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(result.unwrap(), 42, "Expected MPI rank to be 42");
    }

    /// Test that base_data can contain other keys (with number value)
    #[test]
    fn test_extract_base_data_key_with_other_keys_number() {
        let mut lmx_summary: LmxSummary = HashMap::new();
        let mut base_data: HashMap<String, serde_yaml::Value> = HashMap::new();

        // Add other keys to base_data
        base_data.insert(
            "other_key".to_string(),
            serde_yaml::Value::String("other_value".to_string()),
        );
        base_data.insert(
            "my_MPI_rank".to_string(),
            serde_yaml::Value::Number(5.into()),
        );
        base_data.insert(
            "another_key".to_string(),
            serde_yaml::Value::Number(123.into()),
        );
        lmx_summary.insert("base_data".to_string(), base_data);

        let result = extract_base_data_key(&lmx_summary, "my_MPI_rank");

        assert!(result.is_ok(), "Expected successful extraction");
        assert_eq!(
            result.unwrap(),
            5,
            "Expected MPI rank to be 5 regardless of other keys"
        );
    }
}
