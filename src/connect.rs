// Copyright 2026 lmx2db C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
