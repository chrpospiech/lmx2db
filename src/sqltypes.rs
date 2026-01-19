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

use crate::cmdline::CliArgs;
use anyhow::Result;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

pub type SqlTypeHashMap = HashMap<String, HashMap<String, String>>;

#[cfg(test)]
pub(crate) mod create_sqltypes;
#[cfg(test)]
pub(crate) mod sqltype_hashmap;

/// Creates an sqltype file from the database
pub async fn create_sqltype_file(pool: Option<Pool<MySql>>, args: &CliArgs) -> Result<()> {
    let pool = pool.ok_or_else(|| anyhow::anyhow!("Database pool is required"))?;

    if args.verbose || args.dry_run {
        println!("Creating sqltype file: {}", args.sqltypes_file);
    }
    // Get file contents from database
    let hashmap: SqlTypeHashMap = read_sqltypes_from_db(pool, args).await?;
    if args.dry_run {
        println!("Dry run enabled, not writing to file.");
        println!("{:#?}", hashmap);
        std::process::exit(0);
    }
    // Write hashmap to YAML file
    let yaml_string = serde_yaml::to_string(&hashmap).expect("Failed to serialize hashmap to YAML");

    std::fs::write(&args.sqltypes_file, "---\n".to_string() + &yaml_string)
        .expect("Failed to write YAML to file");

    if args.verbose {
        println!("Successfully wrote sqltypes to: {}", args.sqltypes_file);
    }
    // On success:
    Ok(())
}

/// Reads sqltypes from database or file and returns its contents
pub async fn read_sqltypes(pool: Option<Pool<MySql>>, args: &CliArgs) -> Result<SqlTypeHashMap> {
    let pool = match pool {
        Some(p) => p,
        None => {
            eprintln!(
                "No database connection available, reading from file: {}",
                args.sqltypes_file
            );
            return read_sqltypes_from_file(args).await;
        }
    };
    read_sqltypes_from_db(pool, args).await
}

/// Reads sqltypes from database and returns its contents
/// panics on error
pub async fn read_sqltypes_from_db(pool: Pool<MySql>, args: &CliArgs) -> Result<SqlTypeHashMap> {
    if args.verbose || args.dry_run {
        println!("Reading sqltypes from database");
    }
    // Execute "SHOW TABLES" to get all table names
    let tables: Vec<String> =
        sqlx::query_scalar("SHOW FULL TABLES WHERE `Table_type` = 'BASE TABLE'")
            .fetch_all(&pool)
            .await?;

    // Build the result HashMap
    let mut result: SqlTypeHashMap = HashMap::new();

    for table_name in tables {
        if args.verbose || args.dry_run {
            println!("Processing table: {}", table_name);
        }

        // Get columns for this table
        let query = format!("SHOW COLUMNS FROM `{}`", table_name);
        let rows: Vec<(String, String)> = sqlx::query_as(&query).fetch_all(&pool).await?;
        let mut columns: HashMap<String, String> = HashMap::new();
        for (field, field_type) in rows {
            columns.insert(field, field_type);
        }

        result.insert(table_name, columns);
    }

    Ok(result)
}

/// Reads sqltypes from file and returns its contents
/// panics on error
pub async fn read_sqltypes_from_file(args: &CliArgs) -> Result<SqlTypeHashMap> {
    if args.verbose || args.dry_run {
        println!("Reading sqltypes from file: {}", args.sqltypes_file);
    }
    let yaml_string = std::fs::read_to_string(&args.sqltypes_file)?;

    let hashmap: HashMap<String, HashMap<String, String>> = serde_yaml::from_str(&yaml_string)?;

    Ok(hashmap)
}
