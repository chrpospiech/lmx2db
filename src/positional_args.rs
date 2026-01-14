use anyhow::Result;
use glob::glob;
use std::path::Path;

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
