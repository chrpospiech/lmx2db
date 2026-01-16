use crate::cmdline::CliArgs;
use anyhow::Result;
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) mod find_project_file;
#[cfg(test)]
pub(crate) mod project_mockup;

/// Finds a config file by searching up the directory tree from the given file's location.
/// the config file can be args.project_file or args.module_file depending on the boolean
/// argument is_project_file.
/// If the config file argument contains a path separator, it is treated as an absolute
/// or relative path and is used directly.
/// If not, the function searches parent directories for the file.
/// Returns the full path to the project file if found, or an io::Error if not found.
pub fn find_config_file(file_name: &str, args: &CliArgs, is_project_file: bool) -> Result<PathBuf> {
    let config_file = if is_project_file {
        &args.project_file
    } else {
        &args.module_file
    };
    let config_name = if is_project_file { "project" } else { "module" };
    if config_file.contains('/') {
        if args.verbose || args.dry_run {
            println!(
                "Using specified {} file path: '{}'",
                config_name, config_file
            );
        }
        let config_file_path = PathBuf::from(config_file);
        if config_file_path.exists() {
            return Ok(config_file_path);
        } else {
            return Err(anyhow::anyhow!(
                "Required {} file '{}' not found",
                config_name,
                config_file
            ));
        }
    }
    if args.verbose || args.dry_run {
        println!(
            "Searching for {} file '{}' starting from '{}'",
            config_name, config_file, file_name
        );
    }
    let mut current_dir = extract_directory_path(file_name)?;

    loop {
        let config_file_path = current_dir.join(config_file);

        if config_file_path.exists() {
            if args.verbose || args.dry_run {
                println!(
                    "Found {} file at path: '{}'",
                    config_name,
                    config_file_path.to_str().unwrap()
                );
            }
            return Ok(config_file_path);
        }

        if !current_dir.pop() {
            return Err(anyhow::anyhow!(
                "Required {} file '{}' not found in directory tree",
                config_name,
                config_file
            ));
        }
    }
}

///Extracts the directory path from the given file name.
pub fn extract_directory_path(file_name: &str) -> Result<PathBuf> {
    let path = Path::new(file_name);
    let dir_path = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()?;
    Ok(dir_path.to_path_buf())
}

/// Finds the project file by searching up the directory tree from the given file's location.
/// If args.project_file contains a path separator, it is treated as an absolute
/// or relative path and is used directly.
/// If not, the function searches parent directories for the file.
/// Returns the full path to the project file if found, or an io::Error if not found.
pub fn find_project_file(file_name: &str, args: &CliArgs) -> Result<PathBuf> {
    find_config_file(file_name, args, true)
}

/// Finds the module file by searching up the directory tree from the given file's location.
/// If args.module_file contains a path separator, it is treated as an absolute
/// or relative path and is used directly.
/// If not, the function searches parent directories for the file.
/// Returns the full path to the module file if found, or an io::Error if not found.
pub fn find_module_file(file_name: &str, args: &CliArgs) -> Result<PathBuf> {
    find_config_file(file_name, args, false)
}
