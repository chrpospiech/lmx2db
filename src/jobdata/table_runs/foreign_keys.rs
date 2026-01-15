use crate::cmdline::CliArgs;
use crate::jobdata::table_runs::find_file::find_project_file;
use crate::jobdata::LmxSummary;
use anyhow::Result;

#[cfg(test)]
pub(crate) mod import_foreign_keys;
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

/// Function to import foreign keys for the 'runs' table
pub fn import_foreign_keys(
    file_name: &str,
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
        query_list.push(format!(
            "SET @pid = person_id('{}', {});",
            person, do_import
        ));
    } else {
        if args.verbose || args.dry_run {
            println!("Generating person id for user: {}", user_id);
        }
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
pub fn read_project_file(file_name: &str, args: &CliArgs) -> Result<RunsForeignKeys> {
    let project_file_path = find_project_file(file_name, args)?;
    let file_contents = std::fs::read_to_string(&project_file_path)?;
    if args.verbose || args.dry_run {
        println!("Contents of project file:\n{}", file_contents);
    }
    let runs_foreign_keys: RunsForeignKeys = serde_yaml::from_str(&file_contents)?;
    Ok(runs_foreign_keys)
}
