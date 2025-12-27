use crate::cmdline::CliArgs;
use serde_json::Value;
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

    if args.verbose {
        println!("Creating sqlkey file: {}", args.sqlkeys_file);
    }
    // TODO: Implement sqlkey file creation logic
    let _myhashmap: HashMap<String, HashMap<String, Value>> =
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
) -> HashMap<String, HashMap<String, Value>> {
    let pool = match pool {
        Some(p) => p,
        None => {
            eprintln!(
                "Error: No database connection available, reading from file: {}",
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
    _pool: Pool<MySql>,
    args: &CliArgs,
) -> HashMap<String, HashMap<String, Value>> {
    if args.verbose {
        println!("Reading sqlkeys from database");
    }
    HashMap::new()
}

/// Reads sqlkeys from file and returns its contents
/// panics on error
pub async fn read_sqlkeys_from_file(args: &CliArgs) -> HashMap<String, HashMap<String, Value>> {
    if args.verbose {
        println!("Reading sqlkeys from file: {}", args.sqlkeys_file);
    }
    HashMap::new()
}
