use crate::cmdline::CliArgs;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

#[cfg(test)]
pub(crate) mod create_sqltypes;
#[cfg(test)]
pub(crate) mod sqltype_hashmap;

/// Creates an sqltype file from the database
pub async fn create_sqltype_file(
    pool: Option<Pool<MySql>>,
    args: &CliArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = pool.ok_or("Database pool is required")?;

    if args.verbose || args.dry_run {
        println!("Creating sqltype file: {}", args.sqltypes_file);
    }
    // Get file contents from database
    let hashmap: HashMap<String, HashMap<String, String>> = read_sqltypes_from_db(pool, args).await;
    if args.dry_run {
        println!("Dry run enabled, not writing to file.");
        println!("{:#?}", hashmap);
        std::process::exit(0);
    }
    // Write hashmap to YAML file
    let yaml_string = serde_yml::to_string(&hashmap).expect("Failed to serialize hashmap to YAML");

    std::fs::write(&args.sqltypes_file, "---\n".to_string() + &yaml_string)
        .expect("Failed to write YAML to file");

    if args.verbose {
        println!("Successfully wrote sqltypes to: {}", args.sqltypes_file);
    }
    // On success:
    Ok(())
}

/// Reads sqltypes from database or file and returns its contents
pub async fn read_sqltypes(
    pool: Option<Pool<MySql>>,
    args: &CliArgs,
) -> HashMap<String, HashMap<String, String>> {
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
pub async fn read_sqltypes_from_db(
    pool: Pool<MySql>,
    args: &CliArgs,
) -> HashMap<String, HashMap<String, String>> {
    if args.verbose || args.dry_run {
        println!("Reading sqltypes from database");
    }
    // Execute "SHOW TABLES" to get all table names
    let tables: Vec<String> =
        sqlx::query_scalar("SHOW FULL TABLES WHERE `Table_type` = 'BASE TABLE'")
            .fetch_all(&pool)
            .await
            .expect("Failed to fetch tables");

    // Build the result HashMap
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

    for table_name in tables {
        if args.verbose || args.dry_run {
            println!("Processing table: {}", table_name);
        }

        // Get columns for this table
        let query = format!("SHOW COLUMNS FROM `{}`", table_name);
        let err_msg = format!("Failed to fetch columns for table {}", table_name);
        let rows: Vec<(String, String)> = sqlx::query_as(&query)
            .fetch_all(&pool)
            .await
            .expect(&err_msg);
        let mut columns: HashMap<String, String> = HashMap::new();
        for (field, field_type) in rows {
            columns.insert(field, field_type);
        }

        result.insert(table_name, columns);
    }

    result
}

/// Reads sqltypes from file and returns its contents
/// panics on error
pub async fn read_sqltypes_from_file(args: &CliArgs) -> HashMap<String, HashMap<String, String>> {
    if args.verbose || args.dry_run {
        println!("Reading sqltypes from file: {}", args.sqltypes_file);
    }
    let yaml_string =
        std::fs::read_to_string(&args.sqltypes_file).expect("Failed to read sqltypes file");

    let hashmap: HashMap<String, HashMap<String, String>> =
        serde_yml::from_str(&yaml_string).expect("Failed to parse YAML from sqltypes file");

    hashmap
}
