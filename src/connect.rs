use sqlx::mysql::MySqlPool;

pub async fn connect_to_database(database_url: &str) -> Option<MySqlPool> {
    match MySqlPool::connect(database_url).await {
        Ok(pool) => Some(pool),
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            eprintln!("Will output to file instead.");
            None
        }
    }
}

pub async fn disconnect_from_database(pool: Option<MySqlPool>) {
    if let Some(pool) = pool {
        pool.close().await;
    }
}
