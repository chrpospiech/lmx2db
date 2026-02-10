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
    use crate::globbing::find_lmx_type_files;
    use anyhow::Result;

    #[test]
    fn test_invalid_file_name() -> Result<()> {
        // Call the find_lmx_type_files function with an invalid file name
        let result = find_lmx_type_files("invalid_file_name.yml", "typeA");
        // Assert that the result is an error
        assert!(
            result.is_err(),
            "Expected an error for invalid file name, but got: {:?}",
            result
        );
        Ok(())
    }

    #[test]
    fn test_missing_type_files() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir =
            std::env::temp_dir().join(format!("lmx_type_files_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Create a mock LMX_summary file in the temporary directory
        let lmx_summary_path = temp_dir.join("LMX_summary.1234.0.yml");
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

    #[test]
    fn test_existing_type_files() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir =
            std::env::temp_dir().join(format!("lmx_type_files_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Create a mock LMX_summary file in the temporary directory
        let lmx_summary_path = temp_dir.join("LMX_summary.1234.0.yml");
        std::fs::write(&lmx_summary_path, "mock content")?;

        // Create mock LMX_type files in the same directory
        let type_file_1 = temp_dir.join("LMX_typeA_profile.1234.0.yml");
        let type_file_2 = temp_dir.join("LMX_typeA_profile.1234.1.yml");
        let type_file_3 = temp_dir.join("LMX_typeA_profile.5678.1.yml");
        std::fs::write(&type_file_1, "type A content 1")?;
        std::fs::write(&type_file_2, "type A content 2")?;
        std::fs::write(&type_file_3, "type A content 3")?;

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

    #[test]
    fn test_file_without_directory_component() -> Result<()> {
        // Create a temporary directory
        let temp_dir =
            std::env::temp_dir().join(format!("lmx_type_files_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        // Use a guard to ensure cleanup even on test failure
        struct CleanupGuard(std::path::PathBuf);
        impl Drop for CleanupGuard {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let _guard = CleanupGuard(temp_dir.clone());

        // Save and change to temp directory to safely test the edge case
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&temp_dir)?;

        // Use another guard to restore the directory even on panic
        struct DirGuard(std::path::PathBuf);
        impl Drop for DirGuard {
            fn drop(&mut self) {
                let _ = std::env::set_current_dir(&self.0);
            }
        }
        let _dir_guard = DirGuard(original_dir);

        // Create files in the current directory
        let lmx_summary_name = "LMX_summary.1234.0.yml";
        std::fs::write(lmx_summary_name, "mock content")?;

        let type_file_1 = "LMX_typeB_profile.1234.0.yml";
        let type_file_2 = "LMX_typeB_profile.1234.1.yml";
        std::fs::write(type_file_1, "type B content 1")?;
        std::fs::write(type_file_2, "type B content 2")?;

        // Call find_lmx_type_files with just the filename (no directory component)
        // This tests the edge case where parent_dir would be empty
        let files = find_lmx_type_files(lmx_summary_name, "typeB")?;

        // Assert that the result contains the expected files
        assert_eq!(
            files.len(),
            2,
            "Expected 2 matching files, but found: {:?}",
            files
        );

        Ok(())
    }
}
