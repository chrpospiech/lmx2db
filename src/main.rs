use connect::connect_to_database;

pub(crate) mod cmdline;
pub(crate) mod connect;

#[tokio::main]
async fn main() {
    let args = cmdline::parse_args();
    cmdline::echo_args(&args);

    // Connect to the database
    let database_url: String = args.db_url;
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&database_url).await;

    // Explicit disconnect from the database
    pool.close().await;
}
