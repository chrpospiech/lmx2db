use connect::{connect_to_database, disconnect_from_database};
use sqlx::{MySql, Pool};
use std::collections::HashMap;

use crate::positional_args::find_lmx_summary_files;

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod jobdata;
pub(crate) mod positional_args;
pub(crate) mod sqlkeys;

#[tokio::main]
async fn main() {
    let args = cmdline::parse_args();
    cmdline::echo_args(&args);

    // Connect to the database
    let database_url: String = args.db_url.clone();
    let pool: Option<Pool<MySql>> = connect_to_database(&database_url).await;

    // If create_sqlkeys flag is set, create the sqlkey file
    // from the database and exit
    if args.create_sqlkeys {
        match sqlkeys::create_sqlkey_file(pool.clone(), &args).await {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
    // Normal operation: read sqlkeys and proceed
    let sqlkeys: HashMap<String, HashMap<String, String>> =
        sqlkeys::read_sqlkeys(pool.clone(), &args).await;
    if args.verbose || args.dry_run {
        println!("Read {} sqlkeys from database/file", sqlkeys.len());
    }

    // Main loop: process all LMX_SUMMARY files
    for file_name in find_lmx_summary_files(&args.files) {
        if args.verbose {
            println!("Processing file: {}", file_name);
        }
        let return_code = jobdata::process_lmx_file(&file_name, &pool, &sqlkeys, &args).await;
        match return_code {
            Ok(_) => {}
            Err(e) => eprintln!("Ignoring {} because of error: {}", file_name, e),
        }
    }

    // Explicit disconnect from the database
    disconnect_from_database(pool).await;
}
