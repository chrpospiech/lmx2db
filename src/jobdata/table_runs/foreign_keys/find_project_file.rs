#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::foreign_keys::find_project_file;
    use crate::jobdata::table_runs::foreign_keys::project_mockup::{
        setup_cliargs_with_project_file, setup_tmp_project_file, teardown_tmp_project_file,
    };
    use crate::jobdata::table_runs::foreign_keys::RunsForeignKeys;

    #[test]
    fn test_find_project_file_with_absolute_path() {
        // Create a temporary project file for testing
        let temp_dir =
            std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
        let project_file = temp_dir
            .join("absolute_project_file.yml")
            .to_str()
            .unwrap()
            .to_string();
        let project_contents = RunsForeignKeys {
            project: "TestProject".to_string(),
            code: "TestCode".to_string(),
            code_version: "1.0".to_string(),
            test_case: "TestCase".to_string(),
            cluster: None,
            person: None,
        };
        let args = setup_cliargs_with_project_file(&project_file, &project_contents);

        // The file name shouldn't matter here since we provide an absolute path
        // We still call the base name LMX_summary.yml to indicate the kind of file we have in mind
        let result = find_project_file("some/random/path/LMX_summary.yml", &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), project_file);
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_find_project_file_with_base_name() {
        // Create command line arguments for testing
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
        let result = find_project_file(&subdir_file_name, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), project_file);
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_find_project_file_with_relative_file_name() {
        // Create command line arguments for testing
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
        let dirname = std::path::Path::new(&subdir_file_name)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        std::fs::create_dir_all(std::path::Path::new(&subdir_file_name).parent().unwrap())
            .expect("Failed to create subdirectory");
        std::env::set_current_dir(dirname).expect("Failed to change current directory");
        // We start the search from the current directory with a relative path name
        // the file may not exist, we only care for the directories to exist for the search
        let result = find_project_file("LMX_summary.yml", &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), project_file);
        // Clean up
        std::env::set_current_dir(current_dir).expect("Failed to restore current directory");
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_find_project_file_not_found() {
        let args = CliArgs {
            project_file: "non_existent_file.yml".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };

        let result = find_project_file("some/random/path/file.txt", &args);
        assert!(result.is_err());
    }
}
