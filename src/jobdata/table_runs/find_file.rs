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

use crate::cmdline::CliArgs;
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) mod find_project_file;
#[cfg(test)]
pub(crate) mod project_mockup;
#[cfg(test)]
pub(crate) mod read_settings;

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

/// Finds and reads settings file specified in args.settings_file.
/// The file is expected to reside in the same directory of the given file_name.
/// If found, the file is attempted to be read and parsed as YAML
/// returning a HashMap<String, serde_yaml::Value> representing the settings.
/// If not found or if there are issues reading/parsing the file,
/// an appropriate io::Error is returned.
/// A third parameter parameter silent:bool could be added to suppress verbose output,
///
/// #Arguments
/// * `file_name` - Path to the LMX summary file being processed
/// * `args` - Command line arguments controlling processing behavior
/// * `silent` - If true, suppresses verbose output
///
/// #Returns
/// A HashMap<String, serde_yaml::Value> representing the settings from the file
///
/// # Errors
/// - Returns an error if the settings file cannot be found, read, or parsed
pub fn find_and_read_settings_file(
    file_name: &str,
    args: &CliArgs,
    silent: bool,
) -> Result<HashMap<String, serde_yaml::Value>> {
    let dir_path = extract_directory_path(file_name)?;
    let settings_file_path = dir_path.join(&args.settings_file);
    if (args.verbose || args.dry_run) && !silent {
        println!(
            "Looking for settings file at path: '{}'",
            settings_file_path.to_str().unwrap()
        );
    }
    if !settings_file_path.exists() {
        return Err(anyhow::anyhow!(
            "Settings file '{}' not found in directory '{}'",
            args.settings_file,
            dir_path.to_str().unwrap()
        ));
    }
    let file_contents = std::fs::read_to_string(&settings_file_path)?;
    if (args.verbose || args.dry_run) && !silent {
        println!("Contents of settings file:\n{}", file_contents);
    }
    let settings_map: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&file_contents)?;
    Ok(settings_map)
}
