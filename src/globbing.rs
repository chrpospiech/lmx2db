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

use anyhow::{bail, Result};
use glob::glob;
use regex::Regex;
use std::path::Path;

#[cfg(test)]
pub(crate) mod lmx_type_files;

pub fn find_lmx_summary_files(paths: &Vec<String>) -> Result<Vec<String>> {
    let mut result = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);

        if !path.exists() {
            bail!("Path does not exist: {}", path_str);
        }

        if !path.is_dir() {
            bail!("Path is not a directory: {}", path_str);
        }

        let pattern = format!("{}/**/LMX_summary*.yml", path_str.trim_end_matches('/'));

        for entry in glob(&pattern)? {
            let path = entry?;
            if let Some(path_str) = path.to_str() {
                result.push(path_str.to_string());
            }
        }
    }

    Ok(result)
}

/// Function returning the list of files matching
/// format!("LMX_{}_profile.{}*.yml", type_str, process_id)
/// in the same directory as the provided file_name.
/// The typical file_name is an LMX_summary file.
/// The process_id is extracted from the file_name using a
/// regex pattern matching "LMX_summary\.(\d+)\.\d+\.yml".
///
/// # Arguments
/// * `file_name` - The reference file name to determine the directory.
/// * `type_str` - The type string to match in the file names.
///
/// # Returns
/// A Result containing a vector of matching file names or an error.
///
/// Errors if the parent directory cannot be determined or if globbing fails.
///
pub fn find_lmx_type_files(file_name: &str, type_str: &str) -> Result<Vec<String>> {
    let path = Path::new(file_name);
    let parent_dir = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine parent directory of {}", file_name))?;
    let re = Regex::new(r"LMX_summary\.(\d+)\.\d+\.yml").unwrap();
    let process_id = if let Some(caps) = re.captures(file_name) {
        caps.get(1).map_or("", |m| m.as_str())
    } else {
        ""
    };
    if process_id.is_empty() {
        bail!("Cannot extract process ID from file name: {}", file_name);
    }
    let pattern = format!(
        "{}/LMX_{}_profile.{}*.yml",
        parent_dir.display(),
        type_str,
        process_id
    );
    let mut result = Vec::new();
    for entry in glob(&pattern)? {
        let path = entry?;
        if let Some(path_str) = path.to_str() {
            result.push(path_str.to_string());
        }
    }
    Ok(result)
}
