use connect::{connect_to_database, disconnect_from_database};
use sqlx::{MySql, Pool};
use std::collections::HashMap;

use crate::positional_args::find_lmx_summary_files;

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod jobdata;
pub(crate) mod positional_args;
pub(crate) mod sqltypes;

#[tokio::main]
async fn main() {
    let args = cmdline::parse_args();
    cmdline::echo_args(&args);

    // Connect to the database
    let database_url: String = args.db_url.clone();
    let pool: Option<Pool<MySql>> = connect_to_database(&database_url).await;

    // If create_sqltypes flag is set, create the sqltype file
    // from the database and exit
    if args.create_sqltypes {
        match sqltypes::create_sqltype_file(pool.clone(), &args).await {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
    // Normal operation: read sqltypes and proceed
    let sqltypes: HashMap<String, HashMap<String, String>> =
        sqltypes::read_sqltypes(pool.clone(), &args).await;
    if args.verbose || args.dry_run {
        println!("Read {} sqltypes from database/file", sqltypes.len());
    }

    // Main loop: process all LMX_SUMMARY files
    for file_name in find_lmx_summary_files(&args.files) {
        if args.verbose {
            println!("Processing file: {}", file_name);
        }
        let return_code = jobdata::process_lmx_file(&file_name, &pool, &sqltypes, &args).await;
        match return_code {
            Ok(_) => {}
            Err(e) => eprintln!("Ignoring {} because of error:\n     {}", file_name, e),
        }
    }

    // Explicit disconnect from the database
    disconnect_from_database(pool).await;
}
