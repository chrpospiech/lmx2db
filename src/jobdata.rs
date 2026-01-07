use crate::cmdline::CliArgs;
use sqlx::{MySql, Transaction};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub(crate) mod table_runs;

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
/// * `transaction` - Optional ongoing transaction to use for query execution when `args.transaction_per_job` is false
/// * `sqlkeys` - HashMap containing the database schema mapping for generating SQL queries
/// * `args` - Command line arguments controlling processing behavior including verbosity, dry-run mode, and transaction settings
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `sqlx::Error` if database operations fail
///
/// # Behavior
///
/// - Reads and parses the LMX summary file as YAML
/// - Generates SQL queries based on file content and sqlkeys schema mappings
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
    transaction: &mut Option<Transaction<'_, MySql>>,
    sqlkeys: &HashMap<String, HashMap<String, String>>,
    args: &CliArgs,
) -> Result<(), sqlx::Error> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();
    // Adding a comment line as a marker for the file being processed
    query_list.push(format!("-- Queries for file {};", file_name));

    // Read the LMX summary file into a hashmap
    let file_err_msg = format!("Failed to open LMX summary file: {}", file_name);
    let file_content = std::fs::read_to_string(file_name).expect(&file_err_msg);
    let yml_err_msg = format!("Failed to parse YAML from file: {}", file_name);
    let lmx_summary: HashMap<String, serde_yaml::Value> =
        serde_yaml::from_str(&file_content).expect(&yml_err_msg);

    // Generate SQL queries for the 'runs' table
    query_list.extend(table_runs::import_into_runs_table(
        file_name,
        &lmx_summary,
        sqlkeys,
        args,
    ));

    // Process the collected SQL queries
    process_sql_queries(query_list, pool, transaction, args).await?;

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
///
/// # Panics
///
/// Panics if file operations fail in file mode
pub async fn process_sql_queries(
    query_list: Vec<String>,
    pool: &Option<sqlx::Pool<MySql>>,
    transaction: &mut Option<Transaction<'_, MySql>>,
    args: &CliArgs,
) -> Result<(), sqlx::Error> {
    // If args.transaction_per_job is true, create a new transaction for this job.
    let mut tx_per_job = if args.transaction_per_job && !args.dry_run {
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
                } else if let Some(tx) = transaction.as_mut() {
                    sqlx::query(&query).execute(&mut **tx).await?;
                } // else case should not happen as pool is Some
            }
        }
    } else {
        // No database connection available, write (append) them to a file.
        if args.verbose || args.dry_run {
            println!(
                "No database connection available, writing {} lines with queries to file: {}",
                query_list.len(),
                args.sql_file
            );
        }
        if args.dry_run {
            println!("Dry run mode - not writing to file. Queries would be:");
            for query in &query_list {
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
            let mut concatenated = query_list.join("\n");
            // Ensure it ends with a newline
            if !concatenated.ends_with('\n') {
                concatenated.push('\n');
            }
            // Write to the file
            file.write_all(concatenated.as_bytes())
                .expect("Failed to write queries to SQL file");
        }
    }

    // Commit the transaction if using per-job transactions
    if let Some(tx) = tx_per_job {
        tx.commit().await?;
    }
    Ok(())
}
