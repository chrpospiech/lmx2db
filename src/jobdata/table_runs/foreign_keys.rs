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
use crate::jobdata::table_runs::find_file::find_project_file;
use crate::jobdata::LmxSummary;
use anyhow::{bail, Result};
use sqlx::{MySql, Row};

#[cfg(test)]
pub(crate) mod generate_foreign_key_queries;
#[cfg(test)]
pub(crate) mod read_project_file;

/// Struct to hold foreign key data for the runs table
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct RunsForeignKeys {
    pub project: String,
    pub code: String,
    pub code_version: String,
    pub test_case: String,
    pub cluster: Option<String>,
    pub person: Option<String>,
}

/// Helper function to with parameters pool: &Option<sqlx::Pool<MySql>>,
/// query: &String and args: &CliArgs returning Result<()> to execute a query if pool is Some.
/// If pool is None and args.dry_run or args.verbose is set, it prints an informative message.
/// If pool is None and neither args.dry_run nor args.verbose is set, it returns OK without any action.
/// If pool is Some, it executes the query against the database and returns the result which is
/// Result<Option<u64>>. The error is propagated using the ? operator.
/// If the query execution is successful, it checks whether the result is None.
/// If so, it anyhow::bails! with an error message. Otherwise, it returns Ok(()).
///
/// # Arguments
/// * `pool` - Optional reference to a MySQL connection pool
/// * `query` - Reference to the SQL query string to execute
/// * `args` - Reference to command line arguments controlling behavior
///
/// Returns `Result<()>` indicating success or failure of the operation
///
pub async fn execute_query_if_pool(
    pool: &Option<sqlx::Pool<MySql>>,
    query: &String,
    args: &CliArgs,
) -> Result<()> {
    if let Some(db_pool) = pool {
        if args.verbose || args.dry_run {
            println!("Dry run / verbose mode: executing query:\n{}", query);
        }
        let row = sqlx::query(query).fetch_one(db_pool).await?;
        let fetched: Option<u32> = row.try_get::<Option<u32>, _>(0).ok().flatten();
        if fetched.is_none() {
            bail!("Query execution returned no result or NULL");
        }
        Ok(())
    } else {
        if args.verbose || args.dry_run {
            println!("No database pool provided. Skipping execution of foreign key test");
        }
        Ok(())
    }
}

/// Generates SQL queries to set up foreign keys for the runs table based on the provided
/// LMX summary and project file data.
/// This function reads the project file to extract foreign key information,
/// then constructs SQL statements to set up the necessary foreign keys for the runs table.
/// It generates SQL for cluster, person, customer case, filesystem, and duplicate run handling.
///
/// # Arguments
/// * `file_name` - Path to the LMX summary file
/// * `pool` - Optional reference to a MySQL connection pool
/// * `lmx_summary` - Reference to the parsed LMX summary data
/// * `args` - Reference to command line arguments controlling behavior
///
/// # Returns
/// Returns a vector of SQL query strings to set up foreign keys
///
/// # Errors
/// Returns an `anyhow::Error` if there are issues reading the project file or generating queries
///
pub async fn generate_foreign_key_queries(
    file_name: &str,
    pool: &Option<sqlx::Pool<MySql>>,
    lmx_summary: &LmxSummary,
    args: &CliArgs,
) -> Result<Vec<String>> {
    // Collect the SQL queries into a Vec<String> and process them later.
    let mut query_list: Vec<String> = Vec::new();

    // Find a project file and read its RunsForeignKeys
    let foreign_keys = read_project_file(file_name, args)?;

    // Generate SQL statement for cluster foreign key
    let do_import = if args.do_import { "1" } else { "0" };
    let cluster = foreign_keys.cluster.unwrap_or_else(|| "Lenox".to_string());
    if args.verbose || args.dry_run {
        println!("Generating cluster id for cluster: {}", cluster);
    }

    // Before generating the SQL statement, verify the cluster exists if pool is Some
    execute_query_if_pool(
        pool,
        &format!("SELECT cluster_id('{}', {});", cluster, do_import),
        args,
    )
    .await?;
    query_list.push(format!(
        "SET @clid = cluster_id('{}', {});",
        cluster, do_import
    ));

    // Generate SQL statement for person foreign key
    // First extract USER from the LMX summary environment section
    // Fallback to "unknown_user" if not found
    // USER is expected to be a sequence of strings
    // which we join together to form the user id
    let user_id_owned = lmx_summary
        .get("environ")
        .and_then(|m| m.get("USER"))
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("")
        });
    let user_id = user_id_owned.as_deref().unwrap_or("unknown_user");
    // Now generate the SQL statement for person foreign key
    // If person is specified in the project file, use that
    // Otherwise, use the user id from the LMX summary
    if let Some(person) = &foreign_keys.person {
        if args.verbose || args.dry_run {
            println!("Generating person id for person: {}", person);
        }
        execute_query_if_pool(
            pool,
            &format!("SELECT person_id('{}', {});", person, do_import),
            args,
        )
        .await?;
        query_list.push(format!(
            "SET @pid = person_id('{}', {});",
            person, do_import
        ));
    } else {
        if args.verbose || args.dry_run {
            println!("Generating person id for user: {}", user_id);
        }
        execute_query_if_pool(
            pool,
            &format!("SELECT person_id_for_uid('{}', @clid);", user_id),
            args,
        )
        .await?;
        query_list.push(format!(
            "SET @pid = person_id_for_uid('{}', @clid);",
            user_id
        ));
    }

    // Generate SQL statement for customer case foreign key
    if args.verbose || args.dry_run {
        println!(
            "Generating customer case id from project data in file: {}",
            args.project_file
        );
    }
    execute_query_if_pool(
        pool,
        &format!(
            "SELECT customer_case_id('{}', '{}', '{}', '{}', {});",
            foreign_keys.project,
            foreign_keys.code,
            foreign_keys.code_version,
            foreign_keys.test_case,
            do_import
        ),
        args,
    )
    .await?;
    query_list.push(format!(
        "SET @ccid = customer_case_id('{}', '{}', '{}', '{}', {});",
        foreign_keys.project,
        foreign_keys.code,
        foreign_keys.code_version,
        foreign_keys.test_case,
        do_import
    ));

    // Generate SQL statements for filesystem id
    if args.verbose || args.dry_run {
        println!("Generating filesystem id for run directory");
    }
    let base_data = lmx_summary
        .get("base_data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'base_data' in LMX summary"))?;
    let fstype = base_data
        .get("fstype")
        .ok_or_else(|| anyhow::anyhow!("Missing 'fstype' in base_data"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("'fstype' is not a string"))?;
    let m_pt = base_data
        .get("mount_point")
        .ok_or_else(|| anyhow::anyhow!("Missing 'mount_point' in base_data"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("'mount_point' is not a string"))?;
    let bsize = base_data
        .get("blocksize")
        .ok_or_else(|| anyhow::anyhow!("Missing 'blocksize' in base_data"))?
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("'blocksize' is not an integer"))?;
    execute_query_if_pool(
        pool,
        &format!("SELECT filesystem_id('{}', '{}', {});", fstype, m_pt, bsize),
        args,
    )
    .await?;
    query_list.push(format!(
        "SET @fsid = filesystem_id('{}', '{}', {});",
        fstype, m_pt, bsize
    ));

    // Generate SQL drop statement for duplicate runs
    // This uses person_id and timestamps from LMX summary base_data
    // to identify duplicate runs
    if args.verbose || args.dry_run {
        println!("Generating drop statement for duplicate runs");
    }
    let start_date = base_data
        .get("start_date")
        .ok_or_else(|| anyhow::anyhow!("Missing 'start_time' in base_data"))?
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("'start_time' is not an integer"))?;
    let start_date_n = base_data
        .get("start_date_n")
        .ok_or_else(|| anyhow::anyhow!("Missing 'start_time_n' in base_data"))?
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("'start_time_n' is not an integer"))?;
    query_list.push(format!(
        "CALL drop_run_by_user_start_date(@pid, {}, {});",
        start_date, start_date_n
    ));

    // Return the list of generated SQL queries
    Ok(query_list)
}

/// Reads and parses the project file to extract RunsForeignKeys.
/// Returns a RunsForeignKeys struct if successful, or an io::Error if there are issues
/// reading or parsing the file.
///
/// # Arguments
/// * `file_name` - Path to the LMX summary file
/// * `args` - Reference to command line arguments controlling behavior
///
/// # Returns
/// Returns `Result<RunsForeignKeys>` containing the parsed foreign key data
///
pub fn read_project_file(file_name: &str, args: &CliArgs) -> Result<RunsForeignKeys> {
    let project_file_path = find_project_file(file_name, args)?;
    let file_contents = std::fs::read_to_string(&project_file_path)?;
    if args.verbose || args.dry_run {
        println!("Contents of project file:\n{}", file_contents);
    }
    let runs_foreign_keys: RunsForeignKeys = serde_yaml::from_str(&file_contents)?;
    Ok(runs_foreign_keys)
}
