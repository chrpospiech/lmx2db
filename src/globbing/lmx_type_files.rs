#[cfg(test)]
mod tests {
    use crate::globbing::find_lmx_type_files;
    use anyhow::Result;

    #[tokio::test]
    async fn test_missing_type_files() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir =
            std::env::temp_dir().join(format!("lmx_type_files_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Create a mock LMX_summary file in the temporary directory
        let lmx_summary_path = temp_dir.join("LMX_summary.0001.yml");
        std::fs::write(&lmx_summary_path, "mock content")?;

        // Call the find_lmx_type_files function with a type that does not exist
        let files = find_lmx_type_files(lmx_summary_path.to_str().unwrap(), "nonexistent_type")?;

        // Assert that the result is an empty vector
        assert!(
            files.is_empty(),
            "Expected no matching files, but found some: {:?}",
            files
        );

        // Clean up the temporary directory
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    #[tokio::test]
    async fn test_existing_type_files() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir =
            std::env::temp_dir().join(format!("lmx_type_files_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Create a mock LMX_summary file in the temporary directory
        let lmx_summary_path = temp_dir.join("LMX_summary.0001.yml");
        std::fs::write(&lmx_summary_path, "mock content")?;

        // Create mock LMX_type files in the same directory
        let type_file_1 = temp_dir.join("LMX_typeA_001.yml");
        let type_file_2 = temp_dir.join("LMX_typeA_002.yml");
        std::fs::write(&type_file_1, "type A content 1")?;
        std::fs::write(&type_file_2, "type A content 2")?;

        // Call the find_lmx_type_files function with the existing type
        let files = find_lmx_type_files(lmx_summary_path.to_str().unwrap(), "typeA")?;

        // Assert that the result contains the expected files
        assert_eq!(
            files.len(),
            2,
            "Expected 2 matching files, but found: {:?}",
            files
        );
        assert!(
            files.contains(&type_file_1.to_str().unwrap().to_string()),
            "Expected to find file: {:?}",
            type_file_1
        );
        assert!(
            files.contains(&type_file_2.to_str().unwrap().to_string()),
            "Expected to find file: {:?}",
            type_file_2
        );

        // Clean up the temporary directory
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }
}
