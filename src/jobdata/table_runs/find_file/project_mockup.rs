use crate::cmdline::CliArgs;
use crate::jobdata::table_runs::foreign_keys::RunsForeignKeys;

/// Sets up a temporary project file on a new directory for testing purposes.
///
/// This function creates a temporary directory with a unique UUID-based name and
/// then creates a project file inside it with some dummy content.
///
/// # Returns
///
/// Returns the path to the created temporary project file as a `String`.
///
/// # Panics
///
/// Panics if:
/// - The temporary directory cannot be created
/// - The project file cannot be created or written to
pub fn setup_tmp_project_file(args: &CliArgs, contents: &RunsForeignKeys) -> String {
    // Create a temporary project file for testing
    let temp_dir = std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp project file directory");
    let file_name = temp_dir.join(args.project_file.as_str());
    // temporarily create a project file with contents from RunsForeignKeys written in yml format
    let yml_contents =
        serde_yaml::to_string(contents).expect("Failed to serialize RunsForeignKeys to YAML");
    std::fs::write(&file_name, yml_contents).expect("Failed to write to project file");

    file_name.to_str().unwrap().to_string()
}

/// Sets up a CliArgs instance with the specified project file name for testing purposes.
/// Only the name is set; the file itself is not created.
/// However, the necessary directories for the file are created.
///
/// # Arguments
/// * `project_file` - The name of the project file to be set in the CliArgs.
///
/// # Returns
/// A CliArgs instance with the specified project file and default values for other fields.
pub fn setup_cliargs_with_project_file_name(project_file: &str) -> CliArgs {
    // Create directory for the project file
    std::fs::create_dir_all(std::path::Path::new(project_file).parent().unwrap())
        .expect("Failed to create subdirectory");
    // Create CliArgs with the specified project file
    CliArgs {
        project_file: project_file.to_string(),
        verbose: true,
        dry_run: false,
        ..Default::default()
    }
}

/// Sets up a CliArgs instance with the specified project file for testing purposes.
/// This function creates the project file with the provided contents.
///
/// # Arguments
/// * `project_file` - The name of the project file to be set in the CliArgs.
///
/// # Returns
/// A CliArgs instance with the specified project file and default values for other fields.
pub fn setup_cliargs_with_project_file(project_file: &str, contents: &RunsForeignKeys) -> CliArgs {
    // Create directory and CliArgs with the specified project file
    let project_cliargs = setup_cliargs_with_project_file_name(project_file);
    // temporarily create a project file with contents from RunsForeignKeys written in yml format
    let yml_contents =
        serde_yaml::to_string(contents).expect("Failed to serialize RunsForeignKeys to YAML");
    std::fs::write(project_file, yml_contents).expect("Failed to write to project file");
    // Return CliArgs
    project_cliargs
}

/// Cleans up a temporary directory created by `setup_tmp_project_file`.
///
/// # Arguments
///
/// * `temp_file` - Path to the project file created by `setup_tmp_project_file`.
///
/// The parent directory and its contents will be removed.
///
/// # Panics
///
/// Panics if the directory cannot be removed.
pub fn teardown_tmp_project_file(temp_file: &str) {
    std::fs::remove_dir_all(std::path::Path::new(temp_file).parent().unwrap())
        .expect("Failed to remove temp project file directory");
}
