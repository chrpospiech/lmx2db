use crate::cmdline::CliArgs;
use crate::jobdata::create_sql::{create_import_statement, create_update_statement};
use crate::jobdata::LmxSummary;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

pub(crate) mod foreign_keys;

/// Function to import a row into the 'runs' table
/// This function generates the SQL INSERT statement for the 'runs' table
/// based on the provided data and sqltypes.
pub fn import_into_runs_table(
    file_name: &str,
    lmx_summary: &LmxSummary,
    sqltypes: &SqlTypeHashMap,
    args: &CliArgs,
) -> Result<Vec<String>> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();

    query_list.push("-- Inserting into runs table;".to_string());
    if args.verbose || args.dry_run {
        println!("Generating SQL for runs table from file: {}", file_name);
    }

    // Generate SQL queries for foreign keys
    query_list.extend(foreign_keys::import_foreign_keys(
        file_name,
        lmx_summary,
        args,
    )?);

    // Prepare the data for insertion into the 'runs' table
    let mut column_data: Vec<(String, serde_yaml::Value)> = vec![
        (
            "ccid".to_string(),
            serde_yaml::Value::String("@ccid".to_string()),
        ),
        (
            "pid".to_string(),
            serde_yaml::Value::String("@pid".to_string()),
        ),
        (
            "clid".to_string(),
            serde_yaml::Value::String("@clid".to_string()),
        ),
        (
            "fsid".to_string(),
            serde_yaml::Value::String("@fsid".to_string()),
        ),
    ];
    if let Some(runs_section) = lmx_summary.get("base_data") {
        for (key, value) in runs_section {
            let runs_types = sqltypes
                .get("runs")
                .ok_or_else(|| anyhow::anyhow!("'runs' table not found in sqltypes"))?;

            if runs_types.contains_key(key) {
                column_data.push((key.clone(), value.clone()));
            }
        }
    }
    let import_sql = create_import_statement("runs", &column_data, sqltypes)?;
    query_list.push(import_sql);

    // Set @rid for further use
    if args.verbose || args.dry_run {
        println!("Generating rid for current run ");
    }
    query_list.push("SET @rid = LAST_INSERT_ID();".to_string());
    // Create progress indicator
    query_list.push("SELECT concat (\"       rid = \", @rid) as 'Processing run :';".to_string());

    // Compute and import timing information
    if args.verbose || args.dry_run {
        println!("Generating timing information for current run ");
    }
    let timing_data: Vec<(String, serde_yaml::Value)> = vec![
        (
            "collect_time".to_string(),
            serde_yaml::Value::Number(serde_yaml::Number::from(0.1234f64)), // Example value
        ),
        (
            "elapsed".to_string(),
            serde_yaml::Value::Number(serde_yaml::Number::from(12.34f64)), // Example value
        ),
    ];
    // Call create_update_statement for timing table
    let timing_sql = create_update_statement("runs", &timing_data, "rid = @rid", sqltypes)?;
    query_list.push(timing_sql);

    Ok(query_list)
}
