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
    use crate::jobdata::table_mpi::extract_vector_from_serde_yaml;
    use anyhow::Result;
    use serde_yaml::Value;

    #[test]
    fn test_extract_vector_from_serde_yaml() -> Result<()> {
        let yaml_str = r#"
        - item1
        - item2
        - item3
        "#;
        let value: Value = serde_yaml::from_str(yaml_str)?;
        let vec = extract_vector_from_serde_yaml(&value)?;
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], Value::String("item1".to_string()));
        assert_eq!(vec[1], Value::String("item2".to_string()));
        assert_eq!(vec[2], Value::String("item3".to_string()));
        Ok(())
    }

    #[test]
    fn test_extract_vector_from_serde_yaml_not_sequence() {
        let yaml_str = r#"
        key: value
        "#;
        let value: Value = serde_yaml::from_str(yaml_str).unwrap();
        let result = extract_vector_from_serde_yaml(&value);
        assert!(result.is_err());
    }
}
