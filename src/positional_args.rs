use glob::glob;
use std::path::Path;

pub fn find_lmx_summary_files(paths: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);

        if !path.exists() {
            eprintln!("Warning: Ignoring path that does not exist: {}", path_str);
            continue;
        }

        if !path.is_dir() {
            eprintln!(
                "Warning: Ignoring path that is not a directory: {}",
                path_str
            );
            continue;
        }

        let pattern = format!("{}/**/LMX_summary*.yml", path_str.trim_end_matches('/'));

        match glob(&pattern) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            if let Some(path_str) = path.to_str() {
                                result.push(path_str.to_string());
                            }
                        }
                        Err(e) => eprintln!("Error reading glob entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error: Cannot access directory {}: {}", path_str, e),
        }
    }

    result
}
