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

#[cfg(test)]
use anyhow::Result;
use sqlx::MySql;

/// function for testing import of NAMD test data
/// by checking database contents after import.
/// The check is done by calling corresponding
/// functions for each table separately.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
pub async fn check_namd_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // check data in table runs.
    check_namd_runs_data(pool).await?;
    // check data in table settings.
    check_namd_settings_data(pool).await?;
    // check data in table environ.
    check_namd_environ_data(pool).await?;
    Ok(())
}

/// function for testing import of NAMD data in table runs
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
async fn check_namd_runs_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i32, bool, bool, u32)>(
            "SELECT `rid`, `clid`, `pid`, `ccid`, `nodes`, `has_MPItrace`, `has_iprof`, `MPI_ranks` FROM `runs`;"
        )
        .fetch_all(pool)
        .await?;

    // Assert exactly one row was returned
    assert_eq!(
        rows.len(),
        1,
        "Expected exactly 1 row, but got {}",
        rows.len()
    );

    // Assert the values of the returned row
    let (rid, clid, pid, ccid, nodes, has_mpi_trace, has_iprof, mpi_ranks) = &rows[0];
    assert_eq!(*rid, 1);
    assert_eq!(*clid, 1);
    assert_eq!(*pid, 3);
    assert_eq!(*ccid, 1);
    assert_eq!(*nodes, 1);
    assert!(!*has_mpi_trace);
    assert!(!*has_iprof);
    assert_eq!(*mpi_ranks, 8);

    Ok(())
}

/// function for testing import of NAMD data in table settings
/// by checking database contents after import.
/// Since NAMD has no settings.yaml file, this function
/// only checks that no rows were inserted.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_namd_settings_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database - in NAMD case, no rows should be inserted
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM `settings`;")
        .fetch_one(pool)
        .await?;

    // Assert no rows were inserted
    assert_eq!(
        count.0, 0,
        "Expected 0 rows in settings table for NAMD, but got {}",
        count.0
    );

    Ok(())
}

/// function for testing import of NAMD data in table environ
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_namd_environ_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT `rid`, `k`, `value` FROM `environ` WHERE `k` LIKE 'LMX_%' ORDER BY `k`;",
    )
    .fetch_all(pool)
    .await?;

    // Assert exactly four rows were returned
    assert_eq!(
        rows.len(),
        4,
        "Expected exactly 4 rows, but got {}",
        rows.len()
    );

    // Assert the values of the returned rows
    let expected_vars = vec![
        (
            1,
            "LMX_EVENTLIST".to_string(),
            "PAPI_TOT_INS,PAPI_TOT_CYC".to_string(),
        ),
        (1, "LMX_IMBALANCE".to_string(), "1".to_string()),
        (1, "LMX_INTERVAL".to_string(), "20".to_string()),
        (1, "LMX_ITIMERPROF".to_string(), "1".to_string()),
    ];

    for expected in expected_vars {
        assert!(
            rows.contains(&expected),
            "Expected row {:?} not found in database",
            expected
        );
    }

    Ok(())
}
