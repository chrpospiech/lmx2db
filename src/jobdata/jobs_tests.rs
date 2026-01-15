#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::find_file::project_mockup::teardown_tmp_project_file;
    use crate::jobdata::table_runs::foreign_keys::import_foreign_keys;
    use crate::jobdata::{read_lmx_summary, LmxSummary};

    #[tokio::test]
    async fn test_missing_project_file_with_simple_namd_data() {
        // Create a temporary project file for testing
        let temp_dir =
            std::env::temp_dir().join(format!("foreign_key_test_{}", uuid::Uuid::new_v4()));

        // Recursively copy tests/data/NAMD to temp_dir
        let manifest_dir = env!("CARGO_MANIFEST_DIR"); // compile-time
        let path = std::path::Path::new(manifest_dir).join("tests/data/NAMD");
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = true;
        fs_extra::dir::copy(&path, &temp_dir, &options)
            .map_err(std::io::Error::other)
            .expect("Could not copy NAMD test data");
        let project_file = temp_dir.join("project.yml");
        // Create CliArgs with the specified project file
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: true,
            dry_run: false,
            do_import: true,
            ..Default::default()
        };

        // Set the LMX_summary file path and read its contents
        let lmx_summary_pathbuf = temp_dir.join("run_0001/LMX_summary.225250.0.yml");
        let lmx_summary: LmxSummary = read_lmx_summary(lmx_summary_pathbuf.to_str().unwrap())
            .expect("Failed to read LMX summary");

        // Call the import_foreign_keys function
        let result =
            import_foreign_keys(lmx_summary_pathbuf.to_str().unwrap(), &lmx_summary, &args);
        assert!(result.is_err());
        let error_message = format!("{}", result.unwrap_err());
        assert!(
            error_message.contains("Required project file 'not_there.yml' not found"),
            "Unexpected error message: {}",
            error_message
        );
        // Clean up the temporary project file and directory
        teardown_tmp_project_file(project_file.to_str().unwrap());
    }
}
