use connect::{connect_to_database, disconnect_from_database};
use sqlx::{MySql, Pool};
use std::collections::HashMap;

pub(crate) mod cmdline;
pub(crate) mod connect;
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
        sqlkeys::create_sqlkey_file(pool.clone(), &args).await;
    }
    // Normal operation: read sqlkeys and proceed
    let sqlkeys: HashMap<String, HashMap<String, String>> =
        sqlkeys::read_sqlkeys(pool.clone(), &args).await;
    if !args.dry_run {
        println!("{:#?}", sqlkeys);
    }

    // Explicit disconnect from the database
    disconnect_from_database(pool).await;
}
