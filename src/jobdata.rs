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
use anyhow::Result;
use sqlx::MySql;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub type LmxSummary = HashMap<String, HashMap<String, serde_yaml::Value>>;

pub(crate) mod checktypes;
pub(crate) mod create_sql;
pub(crate) mod table_runs;
#[cfg(test)]
pub(crate) mod test_job_failures;
#[cfg(test)]
pub(crate) mod test_sql_file;

/// Processes a single LMX summary file by collecting SQL queries and executing them against a database.
///
/// This function reads an LMX summary YAML file, generates SQL queries based on the content
/// and SQL key mappings, then processes those queries either by executing them against a MySQL
/// database or writing them to a file.
///
/// # Arguments
///
/// * `file_name` - Path to the LMX summary file to process
/// * `pool` - Optional MySQL connection pool for database operations. If `None`, queries are written to a file
/// * `sqltypes` - HashMap containing the database schema mapping for generating SQL queries
/// * `args` - Command line arguments controlling processing behavior including verbosity, dry-run mode, and transaction settings
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `sqlx::Error` if database operations fail
///
/// # Behavior
///
/// - Reads and parses the LMX summary file as YAML
/// - Generates SQL queries based on file content and sqltypes schema mappings
/// - Adds a comment marker identifying the source file being processed
/// - Delegates query execution to `process_sql_queries()` which handles:
///   - Database execution (if pool is provided)
///   - File output (if pool is None)
///   - Transaction management based on `args.transaction_per_job`
///
/// # Panics
///
/// Panics if the file cannot be read or if YAML parsing fails
pub async fn process_lmx_file(
    file_name: &str,
    pool: &Option<sqlx::Pool<MySql>>,
    sqltypes: &HashMap<String, HashMap<String, String>>,
    args: &CliArgs,
) -> Result<()> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();
    // Adding a comment line as a marker for the file being processed
    query_list.push(format!("-- Queries for file {};", file_name));

    // Read the LMX summary file into a hashmap
    let lmx_summary = read_lmx_summary(file_name)?;

    // Generate SQL queries for the 'runs' table
    query_list.extend(
        table_runs::import_into_runs_table(file_name, pool, &lmx_summary, sqltypes, args).await?,
    );

    // Process the collected SQL queries
    process_sql_queries(query_list, pool, args).await?;

    Ok(())
}

/// Processes a collection of SQL queries by either executing them against a database or writing to a file.
///
/// This function handles the actual execution of SQL queries, managing transactions according to
/// the configuration specified in command line arguments.
///
/// # Arguments
///
/// * `query_list` - Vector of SQL query strings to process
/// * `pool` - Optional MySQL connection pool for creating new transactions
/// * `transaction` - Optional ongoing transaction to use for query execution
/// * `args` - Command line arguments controlling transaction and output behavior
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `sqlx::Error` if database operations fail
///
/// # Behavior
///
/// ## Database Mode (when pool is Some)
/// - If `args.transaction_per_job` is true, creates a new transaction for this batch
/// - Otherwise uses the provided transaction
/// - Concatenates and re-splits queries on semicolons to handle fragmented queries
/// - Executes each query and commits the transaction if using per-job mode
/// - **Note:** Each query is expected to end with a semicolon. The function splits on ';'
///   and adds it back, ensuring proper SQL statement termination for sqlx execution.
///
/// ## File Mode (when pool is None)
/// - Appends all queries to the file specified in `args.sql_file`
/// - Creates the file if it doesn't exist
/// - Ensures output ends with a newline
/// - In dry-run mode, prints queries to stdout instead of writing to file
///
/// # Panics
///
pub async fn process_sql_queries(
    query_list: Vec<String>,
    pool: &Option<sqlx::Pool<MySql>>,
    args: &CliArgs,
) -> Result<()> {
    // If neither verbose or dry_run, create a new transaction for this job.
    let mut tx_per_job = if !args.verbose && !args.dry_run {
        if let Some(p) = pool.as_ref() {
            Some(p.begin().await?)
        } else {
            None
        }
    } else {
        None
    };

    // Execute the queries using the appropriate transaction
    if !pool.is_none() {
        if args.verbose || args.dry_run {
            println!("Using database connection for executing queries.");
        }
        // The queries might be scattered across multiple vector elements
        // We have to join the elements into single queries if needed.
        let concatenated = query_list.join(" ");
        let re_arranged_queries: Vec<String> = concatenated
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| format!("{};", s))
            .collect();
        for query in re_arranged_queries {
            if args.verbose || args.dry_run {
                println!("Executing query: {}", query);
            }
            if !args.dry_run {
                if let Some(tx) = tx_per_job.as_mut() {
                    sqlx::query(&query).execute(&mut **tx).await?;
                } // else case should not happen as pool is Some
            }
        }
    } else {
        // No database connection available, write (append) them to a file.
        let mut query_list_with_transaction = vec!["START TRANSACTION;".to_string()];
        query_list_with_transaction.extend(query_list);
        query_list_with_transaction.push("COMMIT;".to_string());
        let extended_query_list = query_list_with_transaction;
        if args.verbose || args.dry_run {
            println!(
                "No database connection available, writing {} lines with queries to file: {}",
                extended_query_list.len(),
                args.sql_file
            );
        }
        if args.dry_run {
            println!("Dry run mode - not writing to file. Queries would be:");
            for query in &extended_query_list {
                println!("Query to write: {}", query);
            }
        } else {
            // Open the file in append mode
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&args.sql_file)
                .expect("Failed to open SQL file for writing");
            // Concatenate all queries into a single string
            let mut concatenated = extended_query_list.join("\n");
            // Ensure it ends with a newline
            if !concatenated.ends_with('\n') {
                concatenated.push('\n');
            }
            // Write to the file
            file.write_all(concatenated.as_bytes())?;
        }
    }

    // Commit the transaction if using per-job transactions
    if let Some(tx) = tx_per_job {
        tx.commit().await?;
    }
    Ok(())
}

fn read_lmx_summary(file_name: &str) -> Result<LmxSummary> {
    let file_content = std::fs::read_to_string(file_name)?;
    let lmx_summary: LmxSummary = serde_yaml::from_str(&file_content)?;
    Ok(lmx_summary)
}
