use crate::cmdline::CliArgs;
use crate::jobdata::create_sql::{create_import_statement, create_update_statement};
use crate::jobdata::table_runs::find_file::extract_directory_path;
use crate::jobdata::table_runs::timing_data::{compute_collect_time, compute_elapsed_time};
use crate::jobdata::table_runs::toolchain::get_toolchain_data;
use crate::jobdata::LmxSummary;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;

pub(crate) mod find_file;
pub(crate) mod foreign_keys;
pub(crate) mod timing_data;
pub(crate) mod toolchain;

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
    // Start with mandatory foreign key columns
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
    // Populate column_data from lmx_summary base_data section
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
    // Import toolchain data from module file and loaded modules
    // This needs to be done before creating the import statement
    // because the runs table doesn't allow default values for these columns.
    // So we need to provide explicit values, even if they are "n/a".
    let current_toolchain = get_toolchain_data(file_name, lmx_summary, args);
    column_data.push((
        "compiler".to_string(),
        if let Some(compiler) = current_toolchain.compiler {
            serde_yaml::Value::String(compiler)
        } else {
            serde_yaml::Value::String("n/a".to_string())
        },
    ));
    column_data.push((
        "compiler_version".to_string(),
        if let Some(compiler_version) = current_toolchain.compiler_version {
            serde_yaml::Value::String(compiler_version)
        } else {
            serde_yaml::Value::String("n/a".to_string())
        },
    ));
    column_data.push((
        "mpilib".to_string(),
        if let Some(mpilib) = current_toolchain.mpilib {
            serde_yaml::Value::String(mpilib)
        } else {
            serde_yaml::Value::String("n/a".to_string())
        },
    ));
    column_data.push((
        "mpilib_version".to_string(),
        if let Some(mpilib_version) = current_toolchain.mpilib_version {
            serde_yaml::Value::String(mpilib_version)
        } else {
            serde_yaml::Value::String("n/a".to_string())
        },
    ));
    // Add the required dirname column as the absolute path
    // of the directory containing the LMX_summary file
    if args.verbose || args.dry_run {
        println!("Adding dirname column to runs table data");
    }
    column_data.push((
        "dirname".to_string(),
        serde_yaml::Value::String(
            extract_directory_path(file_name)?
                .to_str()
                .unwrap()
                .to_string(),
        ),
    ));
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
            compute_collect_time(lmx_summary)?,
        ),
        ("elapsed".to_string(), compute_elapsed_time(lmx_summary)?),
    ];
    // Call create_update_statement for timing table
    let timing_sql = create_update_statement("runs", &timing_data, "rid = @rid", sqltypes)?;
    query_list.push(timing_sql);

    // Dummy call to find_and_read_settings_file to avoid unused import warning
    let _ = find_file::find_and_read_settings_file(file_name, args);

    Ok(query_list)
}
