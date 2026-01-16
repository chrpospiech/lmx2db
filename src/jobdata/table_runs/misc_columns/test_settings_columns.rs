#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::misc_columns::determine_settings_columns;
    use anyhow::Result;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_determine_columns_no_settings_file() -> Result<()> {
        // Assuming the test environment has no settings file in the temp directory
        let temp_dir = std::env::temp_dir().join(format!("no_settings_file_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;
        let lmx_summary_file = temp_dir.join("LMX_summary.yml");
        std::fs::write(&lmx_summary_file, "dummy content")?;
        let result = determine_settings_columns(
            lmx_summary_file.to_str().unwrap(),
            &HashMap::new(),
            &Default::default(),
        );
        let expected: Vec<(String, serde_yaml::Value)> = vec![];
        assert_eq!(result, expected);

        // Clean up
        std::fs::remove_file(&lmx_summary_file)?;
        std::fs::remove_dir(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_determine_columns_with_settings_file() -> Result<()> {
        // Assuming the test environment has a settings file in the temp directory
        let temp_dir = std::env::temp_dir().join(format!("with_settings_file_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;
        let lmx_summary_file = temp_dir.join("LMX_summary.yml");
        std::fs::write(&lmx_summary_file, "dummy content")?;
        let settings_file = temp_dir.join("settings.yml");
        let settings_content = r#"---
compiler: "GNU"
comment: "Test run"
gpus: 2
memory_per_node: 64
"#;
        std::fs::write(&settings_file, settings_content)?;
        let runs_columns = HashMap::from([
            ("gpus".to_string(), "".to_string()),
            ("comment".to_string(), "".to_string()),
            ("compiler".to_string(), "".to_string()),
        ]);
        let args = CliArgs {
            verbose: false,
            dry_run: false,
            settings_file: "settings.yml".to_string(),
            ..Default::default()
        };
        let result =
            determine_settings_columns(lmx_summary_file.to_str().unwrap(), &runs_columns, &args);
        let expected = vec![
            (
                "compiler".to_string(),
                serde_yaml::Value::String("GNU".to_string()),
            ),
            (
                "comment".to_string(),
                serde_yaml::Value::String("Test run".to_string()),
            ),
            (
                "gpus".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(2)),
            ),
        ];
        assert_eq!(result, expected);

        // Clean up
        std::fs::remove_file(&lmx_summary_file)?;
        std::fs::remove_file(&settings_file)?;
        std::fs::remove_dir(&temp_dir)?;

        Ok(())
    }
}
