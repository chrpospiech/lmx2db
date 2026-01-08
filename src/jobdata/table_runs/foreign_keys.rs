use crate::cmdline::CliArgs;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) mod find_project_file;
#[cfg(test)]
pub(crate) mod import_foreign_keys;
#[cfg(test)]
pub(crate) mod project_mockup;
#[cfg(test)]
pub(crate) mod read_project_file;

/// Struct to hold foreign key data for the runs table
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct RunsForeignKeys {
    pub project: String,
    pub code: String,
    pub code_version: String,
    pub test_case: String,
    pub cluster: Option<String>,
    pub person: Option<String>,
}

/// Function to import foreign keys for the 'runs' table
pub fn import_foreign_keys(
    file_name: &str,
    lmx_summary: &HashMap<String, HashMap<String, serde_yaml::Value>>,
    args: &CliArgs,
) -> Vec<String> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();

    // Find a project file and read its RunsForeignKeys
    let foreign_keys = read_project_file(file_name, args)
        .unwrap_or_else(|e| panic!("Failed to read compulsory project file: {}", e));

    // Generate SQL statement for cluster foreign key
    let do_import = if args.do_import { "1" } else { "0" };
    let cluster = foreign_keys.cluster.unwrap_or_else(|| "Lenox".to_string());
    if args.verbose || args.dry_run {
        println!("Generating cluster id for cluster: {}", cluster);
    }
    query_list.push(format!(
        "SET @clid = cluster_id('{}', {});",
        cluster, do_import
    ));

    // Generate SQL statement for person foreign key
    // First extract USER from the LMX summary environment section
    // Fallback to "unknown_user" if not found
    // USER is expected to be a sequence of strings
    // which we join together to form the user id
    let user_id_owned = lmx_summary
        .get("environ")
        .and_then(|m| m.get("USER"))
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("")
        });
    let user_id = user_id_owned.as_deref().unwrap_or("unknown_user");
    // Now generate the SQL statement for person foreign key
    // If person is specified in the project file, use that
    // Otherwise, use the user id from the LMX summary
    if let Some(person) = &foreign_keys.person {
        if args.verbose || args.dry_run {
            println!("Generating person id for person: {}", person);
        }
        query_list.push(format!(
            "SET @pid = person_id('{}', {});",
            person, do_import
        ));
    } else {
        if args.verbose || args.dry_run {
            println!("Generating person id for user: {}", user_id);
        }
        query_list.push(format!(
            "SET @pid = person_id_for_uid('{}', @clid);",
            user_id
        ));
    }

    query_list
}

/// Reads and parses the project file to extract RunsForeignKeys.
/// Returns a RunsForeignKeys struct if successful, or an io::Error if there are issues
/// reading or parsing the file.
pub fn read_project_file(file_name: &str, args: &CliArgs) -> Result<RunsForeignKeys, io::Error> {
    let project_file_path = find_project_file(file_name, args)?;
    let file_contents = std::fs::read_to_string(&project_file_path)?;
    let runs_foreign_keys: RunsForeignKeys = serde_yaml::from_str(&file_contents).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Failed to parse project file '{}': {}",
                project_file_path.display(),
                e
            ),
        )
    })?;
    Ok(runs_foreign_keys)
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
