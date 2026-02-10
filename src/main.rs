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

use anyhow::Result;
use connect::{connect_to_database, disconnect_from_database};
use sqltypes::SqlTypeHashMap;
use sqlx::{MySql, Pool};

use crate::globbing::find_lmx_summary_files;

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod globbing;
pub(crate) mod jobdata;
pub(crate) mod sqltypes;

#[cfg(test)]
mod test_env;

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
    for file_name in find_lmx_summary_files(&args.directories)? {
        println!("Processing file: {}", file_name);
        let return_code = jobdata::process_lmx_file(&file_name, &pool, &sqltypes, &args).await;
        match return_code {
            Ok(_) => {}
            Err(e) => println!("Ignoring {} because of error:\n     {}", file_name, e),
        }
    }

    // Explicit disconnect from the database
    disconnect_from_database(pool).await;
    Ok(())
}
