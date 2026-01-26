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

use crate::cmdline::CliArgs;
use crate::jobdata::create_sql::{create_import_statement, create_update_statement};
use crate::jobdata::table_runs::find_file::extract_directory_path;
use crate::jobdata::table_runs::misc_columns::{
    determine_misc_columns, determine_settings_columns,
};
use crate::jobdata::table_runs::timing_data::import_timing_data;
use crate::jobdata::table_runs::toolchain::import_toolchain_data;
use crate::jobdata::LmxSummary;
use crate::sqltypes::SqlTypeHashMap;
use anyhow::Result;
use sqlx::MySql;

pub(crate) mod find_file;
pub(crate) mod foreign_keys;
pub(crate) mod misc_columns;
pub(crate) mod timing_data;
pub(crate) mod toolchain;

/// Function to import a row into the 'runs' table
/// This function generates the SQL INSERT statement for the 'runs' table
/// based on the provided data and sqltypes.
///
/// # Arguments
/// * `file_name` - Path to the LMX summary file
/// * `pool` - Optional reference to a MySQL connection pool
/// * `lmx_summary` - Reference to the parsed LMX summary data
/// * `sqltypes` - Reference to the SQL types mapping for the database schema
/// * `args` - Reference to command line arguments controlling behavior
///
/// Returns `Result<Vec<String>>` containing the list of SQL queries to execute
///
pub async fn import_into_runs_table(
    file_name: &str,
    pool: &Option<sqlx::Pool<MySql>>,
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
    query_list.extend(
        foreign_keys::generate_foreign_key_queries(file_name, pool, lmx_summary, args).await?,
    );

    // List the columns in the runs table
    let runs_columns = sqltypes
        .get("runs")
        .ok_or_else(|| anyhow::anyhow!("'runs' table not found in sqltypes"))?;

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
    // Populate column_data from lmx_summary base_data section
    if let Some(runs_section) = lmx_summary.get("base_data") {
        for (key, value) in runs_section {
            if runs_columns.contains_key(key) {
                column_data.push((key.clone(), value.clone()));
            }
        }
    }
    // Import toolchain data from module file and loaded modules
    // This needs to be done before creating the import statement
    // because the runs table doesn't allow default values for these columns.
    // So we need to provide explicit values, even if they are "n/a".
    let current_toolchain = import_toolchain_data(file_name, lmx_summary, args);
    column_data.extend(current_toolchain);
    // Convert to new API format
    let keys: Vec<String> = column_data.iter().map(|(k, _)| k.clone()).collect();
    let values: Vec<Vec<serde_yaml::Value>> = vec![column_data.iter().map(|(_, v)| v.clone()).collect()];
    let import_sql = create_import_statement("runs", &keys, &values, sqltypes)?;
    query_list.push(import_sql);

    // Set @rid for further use
    if args.verbose || args.dry_run {
        println!("Generating rid for current run ");
    }
    query_list.push("SET @rid = LAST_INSERT_ID();".to_string());
    // Create progress indicator only for pool.is_none()
    // i.e., when *not* connected to a real database
    // This is useful when importing a file import.sql via
    // the mariadb command line client.
    if pool.is_none() {
        query_list
            .push("SELECT concat (\"       rid = \", @rid) as 'Processing run :';".to_string());
    }

    // Compute and import timing information
    if args.verbose || args.dry_run {
        println!("Generating timing information for current run ");
    }
    let timing_data: Vec<(String, serde_yaml::Value)> = import_timing_data(lmx_summary)?;
    // Call create_update_statement for timing table
    let timing_sql = create_update_statement("runs", &timing_data, "rid = @rid", sqltypes)?;
    query_list.push(timing_sql);

    // Determine miscellaneous columns such as has_MPItrace and has_iprof
    if args.verbose || args.dry_run {
        println!("Determining miscellaneous columns for current run ");
    }
    let mut misc_columns = determine_misc_columns(file_name)?;
    // Append columns from settings file (if any)
    misc_columns.extend(determine_settings_columns(file_name, runs_columns, args));
    // Create update statement for misc columns
    let misc_sql = create_update_statement("runs", &misc_columns, "rid = @rid", sqltypes)?;
    query_list.push(misc_sql);

    Ok(query_list)
}
