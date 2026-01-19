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
    use crate::jobdata::table_runs::find_file::find_and_read_settings_file;

    #[test]
    fn test_no_settings_file() {
        // Create command line arguments for testing
        let args = CliArgs {
            settings_file: "test_settings.yml".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let result = find_and_read_settings_file("some_file_name", &args);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Settings file 'test_settings.yml' not found"));
    }

    #[test]
    fn test_wrong_settings_file() {
        // Create command line arguments for testing
        let args = CliArgs {
            settings_file: "test_settings.yml".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        // Create a temporary settings file with invalid YAML content
        let temp_dir =
            std::env::temp_dir().join(format!("settings_file_test_{}", uuid::Uuid::new_v4()));
        let settings_file_path = temp_dir.join("test_settings.yml");
        std::fs::create_dir_all(&temp_dir).unwrap();
        std::fs::write(&settings_file_path, "invalid_yaml: [unclosed_list").unwrap();

        let result = find_and_read_settings_file(
            settings_file_path.to_str().unwrap().to_string().as_str(),
            &args,
        );
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("did not find expected ',' or ']' at line 2 column 1"));

        // Clean up
        std::fs::remove_file(&settings_file_path).unwrap();
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_valid_settings_file() {
        // Create command line arguments for testing
        let args = CliArgs {
            settings_file: "test_settings.yml".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        // Create a temporary settings file with valid YAML content
        let temp_dir =
            std::env::temp_dir().join(format!("settings_file_test_{}", uuid::Uuid::new_v4()));
        let settings_file_path = temp_dir.join("test_settings.yml");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let yaml_content = r#"---
key1: value1
key2: 42
key3:
  - list_item1
  - list_item2
"#;
        std::fs::write(&settings_file_path, yaml_content).unwrap();

        let result = find_and_read_settings_file(
            settings_file_path.to_str().unwrap().to_string().as_str(),
            &args,
        );
        assert!(result.is_ok());
        let settings_map = result.unwrap();
        assert_eq!(
            settings_map.get("key1").unwrap(),
            &serde_yaml::Value::String("value1".to_string())
        );
        assert_eq!(
            settings_map.get("key2").unwrap(),
            &serde_yaml::Value::Number(42.into())
        );
        assert_eq!(
            settings_map.get("key3").unwrap(),
            &serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("list_item1".to_string()),
                serde_yaml::Value::String("list_item2".to_string())
            ])
        );

        // Clean up
        std::fs::remove_file(&settings_file_path).unwrap();
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
