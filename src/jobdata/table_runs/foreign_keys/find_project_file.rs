#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::foreign_keys::find_project_file;
    use crate::jobdata::table_runs::foreign_keys::project_mockup::{
        setup_cliargs_with_project_file, setup_tmp_project_file, teardown_tmp_project_file,
    };

    #[test]
    fn test_find_project_file_with_absolute_path() {
        // Create a temporary project file for testing
        let temp_dir =
            std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
        let project_file = temp_dir
            .join("absolute_project_file.txt")
            .to_str()
            .unwrap()
            .to_string();
        let args = setup_cliargs_with_project_file(&project_file);

        // The file name shouldn't matter here since we provide an absolute path
        let result = find_project_file("some/random/path/file.yml", &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), project_file);
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_find_project_file_with_base_name() {
        // Create command line arguments for testing
        let args = CliArgs {
            project_file: "test_project_file.txt".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let project_file = setup_tmp_project_file(&args);

        // Create a file name in a subdirectory by replacing "test_project_file.txt" with "subdir1/subdir2/subdir3/test_project_file.txt"
        let subdir_project_file = project_file.replace(
            "test_project_file.txt",
            "subdir1/subdir2/subdir3/test_project_file.txt",
        );
        std::fs::create_dir_all(std::path::Path::new(&subdir_project_file).parent().unwrap())
            .expect("Failed to create subdirectory");
        // the file may not exist, we only care for the directories to exist for the search
        let result = find_project_file(&subdir_project_file, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_str().unwrap(), project_file);
        // Clean up
        teardown_tmp_project_file(&project_file);
    }

    #[test]
    fn test_find_project_file_not_found() {
        let args = CliArgs {
            project_file: "non_existent_file.txt".to_string(),
            verbose: false,
            dry_run: false,
            ..Default::default()
        };

        let result = find_project_file("some/random/path/file.txt", &args);
        assert!(result.is_err());
    }
}
