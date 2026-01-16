use anyhow::Result;
use glob::glob;
use std::path::Path;

#[cfg(test)]
pub(crate) mod lmx_type_files;

pub fn find_lmx_summary_files(paths: &Vec<String>) -> Result<Vec<String>> {
    let mut result = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);

        if !path.exists() {
            anyhow::bail!("Path does not exist: {}", path_str);
        }

        if !path.is_dir() {
            anyhow::bail!("Path is not a directory: {}", path_str);
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

/// Function returning the list of files matching format!("LMX_{}*.yml", type_str)
/// in the same directory as the provided file_name.
/// The typical file_name is an LMX_summary file.
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
    let pattern = format!("{}/LMX_{}*.yml", parent_dir.display(), type_str);
    let mut result = Vec::new();
    for entry in glob(&pattern)? {
        let path = entry?;
        if let Some(path_str) = path.to_str() {
            result.push(path_str.to_string());
        }
    }
    Ok(result)
}

// dummy call to avoid "unused function" warnings
#[allow(dead_code)]
fn _dummy() {
    let _ = find_lmx_summary_files(&vec!["/some/path".to_string()]);
    let _ = find_lmx_type_files("/some/path/LMX_summary.yml", "data");
}
