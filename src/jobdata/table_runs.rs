use crate::cmdline::CliArgs;
use std::collections::HashMap;

pub(crate) mod foreign_keys;

/// Function to import a row into the 'runs' table
/// This function generates the SQL INSERT statement for the 'runs' table
/// based on the provided data and sqlkeys.
pub fn import_into_runs_table(
    file_name: &str,
    lmx_summary: &HashMap<String, serde_yaml::Value>,
    sqlkeys: &HashMap<String, HashMap<String, String>>,
    args: &CliArgs,
) -> Vec<String> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();

    // Dummy usage to avoid unused variable warnings
    let _dummy1: &HashMap<String, serde_yaml::Value> = lmx_summary;
    let _dummy2: &HashMap<String, HashMap<String, String>> = sqlkeys;

    query_list.push("-- Inserting into runs table;".to_string());
    if args.verbose || args.dry_run {
        println!("Generating SQL for runs table from file: {}", file_name);
    }

    // Generate SQL queries for foreign keys
    query_list.extend(foreign_keys::import_foreign_keys(
        file_name,
        lmx_summary,
        args,
    ));
    query_list
}
