#[cfg(test)]
mod tests {
    use crate::jobdata::table_runs::toolchain::get_loaded_modules;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;

    #[test]
    fn test_no_environ_section() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = get_loaded_modules(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "'environ' section not found in LmxSummary"
        );
        Ok(())
    }

    #[test]
    fn test_no_loaded_modules() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
environ:
  SOME_VAR: some_value
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = get_loaded_modules(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Missing 'LOADEDMODULES' in environ section"
        );
        Ok(())
    }

    #[test]
    fn test_loaded_modules_not_sequence() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
environ:
  LOADEDMODULES: "gcc/9.3.0, openmpi/4.0.3, python/3.8.5"
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = get_loaded_modules(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "'LOADEDMODULES' is not a sequence"
        );
        Ok(())
    }

    #[test]
    fn test_loaded_modules_success() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
environ:
  LOADEDMODULES:
    - "gcc/9.3.0:openmpi/4.0."
    - "3:python/3.8.5"
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = get_loaded_modules(&lmx_summary)?;
        let expected_modules = vec![
            "gcc/9.3.0".to_string(),
            "openmpi/4.0.3".to_string(),
            "python/3.8.5".to_string(),
        ];
        assert_eq!(result, expected_modules);
        Ok(())
    }
}
