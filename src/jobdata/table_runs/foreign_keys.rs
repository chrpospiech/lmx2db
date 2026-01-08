use crate::cmdline::CliArgs;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) mod find_project_file;
#[cfg(test)]
pub(crate) mod project_mockup;

/// Struct to hold foreign key data for the runs table
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RunsForeignKeys {
    pub project: String,
    pub code: String,
    pub code_version: String,
    pub test_case: String,
    pub cluster: Option<String>,
    pub person: Option<String>,
}

/// Function to import foreign keys for the 'runs' table
pub fn import_foreign_keys(file_name: &str, args: &CliArgs) -> Vec<String> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();

    // Dummy usage to avoid unused variable warnings
    let _dummy = find_project_file(file_name, args);
    let _dummy2 = RunsForeignKeys {
        project: String::new(),
        code: String::new(),
        code_version: String::new(),
        test_case: String::new(),
        cluster: None,
        person: None,
    };

    query_list.push("-- Inserting foreign keys into runs table;".to_string());
    query_list
}

/// Finds the project file by searching up the directory tree from the given file's location.
/// If the project_file argument contains a path separator, it is treated as an absolute or relative
/// path and is used directly.
/// If not, the function searches parent directories for the file.
/// Returns the full path to the project file if found, or an io::Error if not found.
pub fn find_project_file(file_name: &str, args: &CliArgs) -> Result<PathBuf, io::Error> {
    if args.project_file.contains('/') {
        if args.verbose || args.dry_run {
            println!("Using specified project file path: '{}'", args.project_file);
        }
        let project_file_path = PathBuf::from(&args.project_file);
        if project_file_path.exists() {
            return Ok(project_file_path);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Project file '{}' not found", args.project_file),
            ));
        }
    }
    if args.verbose || args.dry_run {
        println!(
            "Searching for project file '{}' starting from '{}'",
            args.project_file, file_name
        );
    }
    let parent_dir = Path::new(file_name)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut current_dir = parent_dir.canonicalize()?;

    loop {
        let project_file_path = current_dir.join(&args.project_file);

        if project_file_path.exists() {
            return Ok(project_file_path);
        }

        if !current_dir.pop() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Project file '{}' not found in directory tree",
                    args.project_file
                ),
            ));
        }
    }
}
