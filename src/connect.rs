use sqlx::mysql::MySqlPool;

pub async fn connect_to_database(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to the database")
}
