#[cfg(test)]
mod tests {
    use crate::jobdata::table_runs::foreign_keys::import_foreign_keys;
    use crate::jobdata::table_runs::foreign_keys::project_mockup::{
        setup_cliargs_with_project_file_name, teardown_tmp_project_file,
    };
    use std::collections::HashMap;

    #[test]
    fn test_import_foreign_keys_with_simple_namd_data() {
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
        let args = setup_cliargs_with_project_file_name(project_file.to_str().unwrap());

        // Set the LMX_summary file path and read its contents
        let lmx_summary_pathbuf = temp_dir.join("run_0001/LMX_summary.225250.0.yml");
        let lmx_summary_path = lmx_summary_pathbuf.to_str().unwrap();
        let file_err_msg = format!("Failed to open LMX summary file: {}", lmx_summary_path);
        let file_content = std::fs::read_to_string(lmx_summary_path).expect(&file_err_msg);
        let yml_err_msg = format!("Failed to parse YAML from file: {}", lmx_summary_path);
        let lmx_summary: HashMap<String, HashMap<String, serde_yaml::Value>> =
            serde_yaml::from_str(&file_content).expect(&yml_err_msg);

        // Call the import_foreign_keys function
        let sql_queries = import_foreign_keys(lmx_summary_path, &lmx_summary, &args);
        assert_eq!(sql_queries.len(), 2);
        assert_eq!(sql_queries[0], "SET @clid = cluster_id('Lenox', 0);");
        assert_eq!(
            sql_queries[1],
            "SET @pid = person_id_for_uid('xcpospiech', @clid);"
        );

        // Clean up the temporary project file and directory
        teardown_tmp_project_file(project_file.to_str().unwrap());
    }
}
