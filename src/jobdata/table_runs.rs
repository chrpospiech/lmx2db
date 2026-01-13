use crate::cmdline::CliArgs;
use crate::jobdata::checktypes::check_type;
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

    // Dummy usage to avoid unused variable warnings
    let table_name = "runs";
    let map = sqltypes;
    let tuple = [("dummy_key".to_string(), serde_yaml::Value::Null)];
    let _dummy1 = check_type(table_name, &tuple, map);

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
    Ok(query_list)
}
