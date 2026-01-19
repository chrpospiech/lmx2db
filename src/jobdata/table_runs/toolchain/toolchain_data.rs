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

#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::toolchain::get_toolchain_data;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;

    #[test]
    fn test_no_toolchain_map() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            ..Default::default()
        };
        let toolchain = get_toolchain_data("non_existent_file.yml", &lmx_summary, &args);
        assert!(toolchain.is_err());
        Ok(())
    }

    #[test]
    fn test_no_loaded_modules() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
environ:
  SOME_VAR: some_value
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            module_file: "/tmp/test_module_file.yml".to_string(),
            ..Default::default()
        };
        let module_data = r#"
good_compiler:
  compiler: "gcc"
  compiler_version: "9.3.0"
"#;
        std::fs::write("/tmp/test_module_file.yml", module_data)?;
        let toolchain = get_toolchain_data("/tmp/test_module_file.yml", &lmx_summary, &args);
        assert!(toolchain.is_err());
        Ok(())
    }

    #[test]
    fn test_toolchain_extraction() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
environ:
  LOADEDMODULES:
    - "good_compiler"
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            module_file: "/tmp/test_module_file.yml".to_string(),
            ..Default::default()
        };
        let module_data = r#"
good_compiler:
  compiler: "gcc"
  compiler_version: "9.3.0"
"#;
        std::fs::write("/tmp/test_module_file.yml", module_data)?;
        let toolchain = get_toolchain_data("/tmp/test_module_file.yml", &lmx_summary, &args);
        assert!(toolchain.is_ok());
        let toolchain = toolchain.unwrap();
        assert_eq!(toolchain.compiler.unwrap(), "gcc");
        assert_eq!(toolchain.compiler_version.unwrap(), "9.3.0");
        assert!(toolchain.mpilib.is_none());
        assert!(toolchain.mpilib_version.is_none());
        Ok(())
    }
}
