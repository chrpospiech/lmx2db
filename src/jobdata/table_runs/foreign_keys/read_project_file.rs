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
        setup_cliargs_with_project_file_name, setup_tmp_project_file, teardown_tmp_project_file,
    };
    use crate::jobdata::table_runs::foreign_keys::read_project_file;
    use crate::jobdata::table_runs::foreign_keys::RunsForeignKeys;

    #[test]
    fn test_read_project_file_success() {
        // Create a temporary project file for testing
        let args = CliArgs {
            project_file: "test_project_file.yml".to_string(),
            verbose: true,
            dry_run: false,
            ..Default::default()
        };
        let project_contents = RunsForeignKeys {
            project: "TestProject".to_string(),
            code: "TestCode".to_string(),
            code_version: "1.0".to_string(),
            test_case: "TestCase".to_string(),
            cluster: Some("TestCluster".to_string()),
            person: Some("TestPerson".to_string()),
        };
        let project_file = setup_tmp_project_file(&args, &project_contents);

        // Create a file name in a subdirectory by replacing "test_project_file.yml"
        // with "subdir1/subdir2/subdir3/LMX_summary.yml"
        let subdir_file_name = project_file.replace(
            "test_project_file.yml",
            "subdir1/subdir2/subdir3/LMX_summary.yml",
        );
        std::fs::create_dir_all(std::path::Path::new(&subdir_file_name).parent().unwrap())
            .expect("Failed to create subdirectory");
        // the file may not exist, we only care for the directories to exist for the search
        let result = read_project_file(&subdir_file_name, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), project_contents);
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_read_project_file_not_found() {
        // Create command line arguments for testing
        let args = CliArgs {
            project_file: "non_existent_project_file.yml".to_string(),
            verbose: true,
            dry_run: false,
            ..Default::default()
        };
        let file_name = "some_file.yml";
        let result = read_project_file(file_name, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_project_file_invalid_yaml() {
        // Create a temporary project file with invalid YAML content
        let temp_dir =
            std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
        let project_file = temp_dir
            .join("absolute_project_file.yml")
            .to_str()
            .unwrap()
            .to_string();
        // Create CliArgs with the specified project file
        // This also sets up the necessary directories
        let args = setup_cliargs_with_project_file_name(&project_file);
        let invalid_yaml_content = "invalid_yaml: [unclosed_list";
        std::fs::write(&project_file, invalid_yaml_content)
            .expect("Failed to write invalid YAML to project file");
        let result = read_project_file("some/random/path/LMX_summary.yml", &args);
        assert!(result.is_err());
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_read_project_file_incomplete_yaml() {
        // Create a temporary project file with invalid YAML content
        let temp_dir =
            std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
        let project_file = temp_dir
            .join("absolute_project_file.yml")
            .to_str()
            .unwrap()
            .to_string();
        // Create CliArgs with the specified project file
        // This also sets up the necessary directories
        let args = setup_cliargs_with_project_file_name(&project_file);
        let invalid_yaml_content = format!(
            "---\n{}\n{}\n{}\n",
            "project: TestProject", "code: TestCode", "code_version: 1.0"
        );
        std::fs::write(&project_file, invalid_yaml_content)
            .expect("Failed to write invalid YAML to project file");
        let result = read_project_file("some/random/path/LMX_summary.yml", &args);
        assert!(result.is_err());
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_read_project_file_complete_project() {
        // Create a temporary project file with invalid YAML content
        let temp_dir =
            std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
        let project_file = temp_dir
            .join("absolute_project_file.yml")
            .to_str()
            .unwrap()
            .to_string();
        // Create CliArgs with the specified project file
        // This also sets up the necessary directories
        let args = setup_cliargs_with_project_file_name(&project_file);
        let invalid_yaml_content = format!(
            "---\n{}\n{}\n{}\n{}\n",
            "project: TestProject", "code: TestCode", "code_version: 1.0", "test_case: TestCase"
        );
        std::fs::write(&project_file, invalid_yaml_content)
            .expect("Failed to write invalid YAML to project file");
        let result = read_project_file("some/random/path/LMX_summary.yml", &args);
        assert!(result.is_ok());
        let runs_foreign_keys = result.unwrap();
        assert_eq!(runs_foreign_keys.project, "TestProject");
        assert_eq!(runs_foreign_keys.code, "TestCode");
        assert_eq!(runs_foreign_keys.code_version, "1.0");
        assert_eq!(runs_foreign_keys.test_case, "TestCase");
        assert!(runs_foreign_keys.cluster.is_none());
        assert!(runs_foreign_keys.person.is_none());
        // Clean up
        teardown_tmp_project_file(&project_file);
    }
}
