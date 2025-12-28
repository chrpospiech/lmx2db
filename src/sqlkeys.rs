use crate::cmdline::CliArgs;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

/// Creates an sqlkey file from the database
pub async fn create_sqlkey_file(pool: Option<Pool<MySql>>, args: &CliArgs) {
    let pool = match pool {
        Some(p) => p,
        None => {
            eprintln!("Error: Database pool is required");
            std::process::exit(1);
        }
    };

    if args.verbose || args.dry_run {
        println!("Creating sqlkey file: {}", args.sqlkeys_file);
    }
    // TODO: Implement sqlkey file creation logic
    let _myhashmap: HashMap<String, HashMap<String, String>> =
        read_sqlkeys_from_db(pool, args).await;
    // On success:
    std::process::exit(0);

    // On error (example):
    // eprintln!("Error: Failed to create sqlkey file");
    // std::process::exit(1);
}

/// Reads sqlkeys from database or file and returns its contents
/// panics on error
pub async fn read_sqlkeys(
    pool: Option<Pool<MySql>>,
    args: &CliArgs,
) -> HashMap<String, HashMap<String, String>> {
    let pool = match pool {
        Some(p) => p,
        None => {
            eprintln!(
                "No database connection available, reading from file: {}",
                args.sqlkeys_file
            );
            return read_sqlkeys_from_file(args).await;
        }
    };
    read_sqlkeys_from_db(pool, args).await
}

/// Reads sqlkeys from database and returns its contents
/// panics on error
pub async fn read_sqlkeys_from_db(
    pool: Pool<MySql>,
    args: &CliArgs,
) -> HashMap<String, HashMap<String, String>> {
    if args.verbose || args.dry_run {
        println!("Reading sqlkeys from database");
    }
    // Execute "SHOW TABLES" to get all table names
    let tables: Vec<String> =
        sqlx::query_scalar("SHOW FULL TABLES WHERE `Table_type` = 'BASE TABLE'")
            .fetch_all(&pool)
            .await
            .expect("Failed to fetch tables");

    // Build the result HashMap
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

    for table_name in tables {
        if args.verbose || args.dry_run {
            println!("Processing table: {}", table_name);
        }

        // Get columns for this table
        let query = format!("SHOW COLUMNS FROM `{}`", table_name);
        let err_msg = format!("Failed to fetch columns for table {}", table_name);
        let rows: Vec<(String, String)> = sqlx::query_as(&query)
            .fetch_all(&pool)
            .await
            .expect(&err_msg);
        let mut columns: HashMap<String, String> = HashMap::new();
        for (field, field_type) in rows {
            columns.insert(field, field_type);
        }

        result.insert(table_name, columns);
    }

    result
}

/// Reads sqlkeys from file and returns its contents
/// panics on error
pub async fn read_sqlkeys_from_file(args: &CliArgs) -> HashMap<String, HashMap<String, String>> {
    if args.verbose || args.dry_run {
        println!("Reading sqlkeys from file: {}", args.sqlkeys_file);
    }
    HashMap::new()
}
