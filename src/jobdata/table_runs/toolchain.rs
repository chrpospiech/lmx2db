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
use crate::jobdata::table_runs::find_file::find_module_file;
use crate::jobdata::LmxSummary;
use anyhow::Result;
use std::collections::HashMap;

#[cfg(test)]
pub(crate) mod loaded_modules;
#[cfg(test)]
pub(crate) mod toolchain_data;

/// Struct to hold foreign key data for the runs table
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ToolChain {
    pub compiler: Option<String>,
    pub compiler_version: Option<String>,
    pub mpilib: Option<String>,
    pub mpilib_version: Option<String>,
}

pub type ToolChainMap = HashMap<String, ToolChain>;

/// Function to import toolchain data for the 'runs' table
/// This function reads the module file and extracts toolchain information
/// based on the loaded modules in the LMX summary.
///
/// Arguments:
/// * `file_name` - The name of the job file
/// * `lmx_summary` - The LMX summary data
/// * `args` - Command line arguments
///
/// Returns:
/// * `Vec<(String, serde_yaml::Value)>` - The extracted toolchain data as column-value pairs
///
/// Errors:
/// This function will never return an error - any errors encountered during
/// file reading or parsing are caught and logged, and an empty ToolChain is returned instead.
///
pub fn import_toolchain_data(
    file_name: &str,
    lmx_summary: &LmxSummary,
    args: &CliArgs,
) -> Vec<(String, serde_yaml::Value)> {
    // Initialize default or returned data
    let empty_toolchain = ToolChain {
        compiler: None,
        compiler_version: None,
        mpilib: None,
        mpilib_version: None,
    };
    let mut column_data: Vec<(String, serde_yaml::Value)> = Vec::new();
    let toolchain = match get_toolchain_data(file_name, lmx_summary, args) {
        Ok(toolchain) => toolchain,
        Err(e) => {
            if args.verbose || args.dry_run {
                println!("Ignoring: {}", e);
            }
            empty_toolchain.clone()
        }
    };
    let compiler = toolchain.compiler.unwrap_or_else(|| "n/a".to_string());
    let compiler_version = toolchain
        .compiler_version
        .unwrap_or_else(|| "n/a".to_string());
    let mpilib = toolchain.mpilib.unwrap_or_else(|| "n/a".to_string());
    let mpilib_version = toolchain
        .mpilib_version
        .unwrap_or_else(|| "n/a".to_string());
    if args.verbose || args.dry_run {
        println!("Toolchain data extracted:");
        println!("  Compiler: {}", compiler);
        println!("  Compiler Version: {}", compiler_version);
        println!("  MPI Library: {}", mpilib);
        println!("  MPI Library Version: {}", mpilib_version);
    }
    column_data.push(("compiler".to_string(), serde_yaml::Value::String(compiler)));
    column_data.push((
        "compiler_version".to_string(),
        serde_yaml::Value::String(compiler_version),
    ));
    column_data.push(("mpilib".to_string(), serde_yaml::Value::String(mpilib)));
    column_data.push((
        "mpilib_version".to_string(),
        serde_yaml::Value::String(mpilib_version),
    ));
    column_data
}

/// Function to get toolchain data for the 'runs' table
/// This function reads the module file and extracts toolchain information
/// based on the loaded modules in the LMX summary.
///
/// Arguments:
/// * `file_name` - The name of the job file
/// * `lmx_summary` - The LMX summary data
/// * `args` - Command line arguments
///
/// Returns:
/// * `Result<ToolChain>` - The extracted toolchain data
///
/// Errors:
/// This function will return an error if there are issues reading or parsing the module file,
/// or if there are issues retrieving the loaded modules from the LMX summary.
///
pub fn get_toolchain_data(
    file_name: &str,
    lmx_summary: &LmxSummary,
    args: &CliArgs,
) -> Result<ToolChain> {
    // Initialize an empty ToolChain
    let mut current_toolchain = ToolChain {
        compiler: None,
        compiler_version: None,
        mpilib: None,
        mpilib_version: None,
    };
    let toolchain_map = read_module_file(file_name, args)?;
    let loaded_modules = get_loaded_modules(lmx_summary)?;
    for module in &loaded_modules {
        if let Some(toolchain) = toolchain_map.get(module) {
            if let Some(ref compiler) = toolchain.compiler {
                current_toolchain.compiler = Some(compiler.clone());
            }
            if let Some(ref compiler_version) = toolchain.compiler_version {
                current_toolchain.compiler_version = Some(compiler_version.clone());
            }
            if let Some(ref mpilib) = toolchain.mpilib {
                current_toolchain.mpilib = Some(mpilib.clone());
            }
            if let Some(ref mpilib_version) = toolchain.mpilib_version {
                current_toolchain.mpilib_version = Some(mpilib_version.clone());
            }
        }
    }

    Ok(current_toolchain)
}

/// Reads and parses the module file to extract toolchain information.
/// Returns a ToolChainMap if successful, or an Error if there are issues
/// reading or parsing the file.
pub fn read_module_file(file_name: &str, args: &CliArgs) -> Result<ToolChainMap> {
    let module_file_path = find_module_file(file_name, args)?;
    let file_contents = std::fs::read_to_string(&module_file_path)?;
    if args.verbose || args.dry_run {
        println!("Contents of module file:\n{}", file_contents);
    }
    let toolchain_map: ToolChainMap = serde_yaml::from_str(&file_contents)?;
    Ok(toolchain_map)
}

/// Retrieves the list of loaded modules from LMX summary data.
///
/// This function extracts the `environ` section from the provided
/// LMX summary and retrieves the value associated with the `LOADEDMODULES` key.
/// The value is expected to be a sequence of strings, which are joined together
/// and then split by ':' to form the final list of loaded modules.
///
/// # Arguments
///
/// * `lmx_summary` - A reference to the LMX summary data containing timing information
///
/// # Returns
///
/// * `Result<Vec<String>>` - The list of loaded modules as strings
///
/// # Errors
///
/// This function will return an error if:
/// * The `environ` section is not found in the LMX summary
/// * The `LOADEDMODULES` key is not found in the `environ` section
/// * The value associated with `LOADEDMODULES` is not a sequence of strings
pub fn get_loaded_modules(lmx_summary: &LmxSummary) -> Result<Vec<String>> {
    let loaded_modules_owned = lmx_summary
        .get("environ")
        .ok_or_else(|| anyhow::anyhow!("'environ' section not found in LmxSummary"))?
        .get("LOADEDMODULES")
        .ok_or_else(|| anyhow::anyhow!("Missing 'LOADEDMODULES' in environ section"))?
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("'LOADEDMODULES' is not a sequence"))?
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| anyhow::anyhow!("Module name is not a string"))
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<String>>>()?;
    let loaded_modules: Vec<String> = loaded_modules_owned
        .join("")
        .split(':')
        .map(|s| s.to_string())
        .collect();
    Ok(loaded_modules)
}
