use anyhow::Result;
use connect::{connect_to_database, disconnect_from_database};
use sqltypes::SqlTypeHashMap;
use sqlx::{MySql, Pool};

use crate::positional_args::find_lmx_summary_files;

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod jobdata;
pub(crate) mod positional_args;
pub(crate) mod sqltypes;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmdline::parse_args();
    cmdline::echo_args(&args);

    // Connect to the database
    let database_url: String = args.db_url.clone();
    let pool: Option<Pool<MySql>> = connect_to_database(&database_url).await;

    // If create_sqltypes flag is set, create the sqltype file
    // from the database and exit
    if args.create_sqltypes {
        sqltypes::create_sqltype_file(pool.clone(), &args).await?;
        std::process::exit(0);
    }
    // Normal operation: read sqltypes and proceed
    let sqltypes: SqlTypeHashMap = sqltypes::read_sqltypes(pool.clone(), &args).await?;
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
    Ok(())
}
