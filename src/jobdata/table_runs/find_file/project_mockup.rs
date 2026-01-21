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
use crate::jobdata::table_runs::foreign_keys::RunsForeignKeys;
use anyhow::Result;
use std::path::PathBuf;
use fs_extra;

/// Sets up a temporary project file on a new directory for testing purposes.
///
/// This function creates a temporary directory with a unique UUID-based name and
/// then creates a project file inside it with some dummy content.
///
/// # Returns
///
/// Returns the path to the created temporary project file as a `String`.
///
/// # Errors
///
/// Returns an error if:
/// - The temporary directory cannot be created
/// - The project file cannot be created or written to
/// - The file path cannot be converted to a valid UTF-8 string
pub fn setup_tmp_project_file(args: &CliArgs, contents: &RunsForeignKeys) -> Result<String> {
    // Create a temporary project file for testing
    let temp_dir = std::env::temp_dir().join(format!("project_file_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir)?;
    let file_name = temp_dir.join(args.project_file.as_str());
    // temporarily create a project file with contents from RunsForeignKeys written in yml format
    let yml_contents = serde_yaml::to_string(contents)?;
    std::fs::write(&file_name, yml_contents)?;

    let file_name_str = file_name
        .into_os_string()
        .into_string()
        .map_err(|os_str| anyhow::anyhow!("temporary project file path is not valid UTF-8: {:?}", os_str))?;

    Ok(file_name_str)
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
pub fn setup_cliargs_with_project_file_name(project_file: &str) -> Result<CliArgs> {
    // Create directory for the project file
    let path = std::path::Path::new(project_file);
    let parent = path.parent().ok_or_else(|| {
        anyhow::anyhow!("Project file '{}' has no parent directory", project_file)
    })?;
    std::fs::create_dir_all(parent)?;
    // Create CliArgs with the specified project file
    Ok(CliArgs {
        project_file: project_file.to_string(),
        verbose: false,
        dry_run: false,
        ..Default::default()
    })
}

/// Sets up a CliArgs instance with the specified project file for testing purposes.
/// This function creates the project file with the provided contents.
///
/// # Arguments
/// * `project_file` - The name of the project file to be set in the CliArgs.
///
/// # Returns
/// A CliArgs instance with the specified project file and default values for other fields.
pub fn setup_cliargs_with_project_file(
    project_file: &str,
    contents: &RunsForeignKeys,
) -> Result<CliArgs> {
    // Create directory and CliArgs with the specified project file
    let project_cliargs = setup_cliargs_with_project_file_name(project_file)?;
    // temporarily create a project file with contents from RunsForeignKeys written in yml format
    let yml_contents =
        serde_yaml::to_string(contents).expect("Failed to serialize RunsForeignKeys to YAML");
    std::fs::write(project_file, yml_contents)?;
    // Return CliArgs
    Ok(project_cliargs)
}

/// sets up a temporary directory for testing purposes, copied from a given source path.
/// The directory is copied to a new temporary location with a unique UUID-based name.
/// This might include a given project file inside it.
///
/// # Arguments
/// * `source_path` - The source path relative to the Cargo manifest directory.
///
/// # Returns
/// Returns the path to the created temporary directory as a `PathBuf`.
///
/// # Errors
/// Returns an error if:
/// - The temporary directory cannot be created
/// - The source directory cannot be copied
pub fn setup_tmp_project_directory(source_path: &str) -> Result<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // compile-time
    let path = std::path::Path::new(manifest_dir).join(source_path);
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("project_dir_test_{}", uuid::Uuid::new_v4()));
    // Recursively copy source_path to temp_dir
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;
    fs_extra::dir::copy(&path, &temp_dir, &options)?;
    Ok(temp_dir)
}

/// Cleans up a temporary directory as created by previous functions.
///
/// # Arguments
///
/// * `temp_file` - Path to the project file created by `setup_tmp_project_file`.
///
/// The parent directory and its contents will be removed.
///
/// # Errors
///
/// Returns an error if:
/// - The temporary file has no parent directory
/// - The directory cannot be removed
pub fn teardown_tmp_project_file(temp_file: &str) -> Result<()> {
    let path = std::path::Path::new(temp_file);
    let parent = path.parent().ok_or_else(|| {
        anyhow::anyhow!("Temporary file '{}' has no parent directory", temp_file)
    })?;
    std::fs::remove_dir_all(parent)?;
    Ok(())
}
