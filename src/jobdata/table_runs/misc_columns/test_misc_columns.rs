#[cfg(test)]
mod tests {
    use crate::jobdata::table_runs::misc_columns::determine_misc_columns;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_determine_misc_columns_no_type_files() -> Result<()> {
        // Assuming the test environment has no type files in the temp directory
        let temp_dir = std::env::temp_dir().join(format!("no_type_files_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let lmx_summary_file = temp_dir.join("LMX_summary.yml");
        std::fs::write(&lmx_summary_file, "dummy content").unwrap();
        let result = determine_misc_columns(lmx_summary_file.to_str().unwrap())?;
        let expected = vec![
            (
                "has_MPItrace".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(0)),
            ),
            (
                "has_iprof".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(0)),
            ),
        ];
        assert_eq!(result, expected);

        // Clean up
        std::fs::remove_file(&lmx_summary_file)?;
        std::fs::remove_dir(&temp_dir)?;

        Ok(())
    }

    #[test]
    fn test_determine_misc_columns_with_type_files() -> Result<()> {
        // Assuming the test environment has type files in the temp directory
        let temp_dir = std::env::temp_dir().join(format!("with_type_files_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let lmx_summary_file = temp_dir.join("LMX_summary.yml");
        std::fs::write(&lmx_summary_file, "dummy content")?;
        // Create dummy type files
        let mpi_type_file = temp_dir.join("LMX_MPI_type_file.yml");
        let iprof_type_file = temp_dir.join("LMX_itimer_type_file.yml");
        std::fs::write(&mpi_type_file, "dummy MPI content")?;
        std::fs::write(&iprof_type_file, "dummy itimer content")?;
        let result = determine_misc_columns(lmx_summary_file.to_str().unwrap())?;
        let expected = vec![
            (
                "has_MPItrace".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(1)),
            ),
            (
                "has_iprof".to_string(),
                serde_yaml::Value::Number(serde_yaml::Number::from(1)),
            ),
        ];
        assert_eq!(result, expected);

        // Clean up
        std::fs::remove_file(&lmx_summary_file)?;
        std::fs::remove_file(&mpi_type_file)?;
        std::fs::remove_file(&iprof_type_file)?;
        std::fs::remove_dir(&temp_dir)?;

        Ok(())
    }
}
