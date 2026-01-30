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
    use crate::{
        cmdline::CliArgs, jobdata::table_tasks::import_into_tasks_table, sqltypes::read_sqltypes,
    };
    use anyhow::Result;
    use sqlx::MySql;
    use std::collections::HashMap;

    /// Test early exit when 'tasks' table is not in sqltypes
    #[test]
    fn test_import_tasks_missing_table() -> Result<()> {
        let args = CliArgs {
            project_file: "project.yml".to_string(),
            settings_file: "settings.yml".to_string(),
            module_file: "modules.yml".to_string(),
            do_import: true,
            dry_run: false,
            verbose: false,
            ..Default::default()
        };

        // Create an empty sqltypes map (no 'tasks' table)
        let sqltypes: HashMap<String, HashMap<String, String>> = HashMap::new();

        // Create a minimal LMX summary with required sections
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lmx_file = std::path::Path::new(manifest_dir)
            .join("tests/data/GROMACS/run_64/LMX_summary.376231.0.yml");
        let lmx_summary: crate::jobdata::LmxSummary =
            serde_yaml::from_str(&std::fs::read_to_string(&lmx_file)?)?;

        // Call import_into_tasks_table with no 'tasks' table in sqltypes
        let queries = import_into_tasks_table(&lmx_summary, &sqltypes, &args)?;

        // Should return empty vector without processing tasks
        assert!(
            queries.is_empty(),
            "Expected empty query list when 'tasks' table is not in sqltypes"
        );

        Ok(())
    }
}
