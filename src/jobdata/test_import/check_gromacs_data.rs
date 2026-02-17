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
use sqlx::MySql;

/// Function for testing import of GROMACS test data
/// by checking database contents after import.
/// The check is done by calling corresponding
/// functions for each table separately.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
pub async fn check_gromacs_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Check data in table runs.
    check_gromacs_runs_data(pool).await?;
    // Check data in table settings.
    check_gromacs_settings_data(pool).await?;
    // Check data in table environ.
    check_gromacs_environ_data(pool).await?;
    // Check data in table mmm.
    check_gromacs_mmm_data(pool).await?;
    // Check data in table tasks.
    check_gromacs_tasks_data(pool).await?;
    // Check data in table mpi.
    check_gromacs_mpi_data(pool).await?;
    // Check data in table mpi_details.
    check_gromacs_mpi_details_data(pool).await?;
    // Check data in table iprof.
    check_gromacs_iprof_data(pool).await?;
    Ok(())
}

/// Function for testing import of GROMACS data in table runs
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
async fn check_gromacs_runs_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
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
    assert_eq!(*pid, 1);
    assert_eq!(*ccid, 2);
    assert_eq!(*nodes, 1);
    assert!(*has_mpi_trace);
    assert!(*has_iprof);
    assert_eq!(*mpi_ranks, 64);

    Ok(())
}

/// Function for testing import of GROMACS data in table settings
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_settings_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database for performance-related settings
    // These should have been imported into table `runs`, so the count should be zero.
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM `settings` WHERE `k` LIKE 'perf_%';")
        .fetch_one(pool)
        .await?;
    // Assert no performance-related settings are present
    assert_eq!(
        count.0, 0,
        "Expected 0 performance-related settings, but got {}",
        count.0
    );
    // Query the database again
    let rows = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT `rid`, `k`, `value` FROM `settings` WHERE `k` LIKE 'PME%' ORDER BY `k`;",
    )
    .fetch_all(pool)
    .await?;

    // Assert exactly two rows were returned
    assert_eq!(
        rows.len(),
        2,
        "Expected exactly 2 rows, but got {}",
        rows.len()
    );

    // Assert the values of the returned rows
    let expected_vars = vec![
        (1, "PME".to_string(), "24".to_string()),
        (1, "PME_DD".to_string(), "24x1x1".to_string()),
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

/// Function for testing import of GROMACS data in table environ
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_environ_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT `rid`, `k`, `value` FROM `environ` WHERE `k` LIKE 'LMX_%' ORDER BY `k`;",
    )
    .fetch_all(pool)
    .await?;

    // Assert exactly three rows were returned
    assert_eq!(
        rows.len(),
        3,
        "Expected exactly 3 rows, but got {}",
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

/// Function for testing import of GROMACS data in table mmm
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_mmm_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i64, i64, f64, Option<i64>, Option<f64>)>(
        "SELECT `rid`, `mintask`, `mincomm`, `minmpiiotask`, `minmpiio` FROM `mmm`;",
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
    let (rid, mintask, mincomm, minmpiiotask, minmpiio) = &rows[0];
    assert_eq!(*rid, 1);
    assert_eq!(*mintask, 30);
    assert!(
        (*mincomm - 216.5484).abs() < 1e-4,
        "mincomm was {}, expected 216.5484",
        mincomm
    );
    assert!(minmpiiotask.is_none());
    assert!(minmpiio.is_none());

    Ok(())
}

/// function for testing import of GROMACS data in table tasks
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_tasks_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (u32, i32)>(
        "SELECT `tid`, CONVERT(`systime`, SIGNED) FROM `tasks` WHERE `tid` < 8 ORDER BY `tid`;",
    )
    .fetch_all(pool)
    .await?;

    // Assert exactly 8 rows were returned
    assert_eq!(
        rows.len(),
        8,
        "Expected exactly 8 rows, but got {}",
        rows.len()
    );

    // Assert the values of the returned rows
    let expected_tasks = vec![
        (0, 22),
        (1, 43),
        (2, 76),
        (3, 80),
        (4, 73),
        (5, 35),
        (6, 22),
        (7, 73),
    ];

    for expected in expected_tasks {
        assert!(
            rows.contains(&expected),
            "Expected row {:?} not found in database",
            expected
        );
    }

    Ok(())
}

/// Function for testing import of GROMACS data in table mpi
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_mpi_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i32, f32, f32)>(
        "SELECT `calls`, `avgbytes`, `time` FROM `mpi` WHERE `tid` = 0 AND mid = 1;",
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
    let (calls, avgbytes, time) = &rows[0];
    assert_eq!(*calls, 2);
    assert!(
        (*avgbytes - 66.0000).abs() < 1e-4,
        "avgbytes was {}, expected 66.0000",
        avgbytes
    );
    assert!(
        (*time - 5.00679e-06).abs() < 1e-4,
        "time was {}, expected 5.00679e-06",
        time
    );

    Ok(())
}

/// Function for testing import of GROMACS data in table mpi_details
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_mpi_details_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i32, f32, f32)>(
        "SELECT `calls`, `avgbytes`, `time` FROM `mpi_details` WHERE `tid` = 30 AND mid = 1;",
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
    let (calls, avgbytes, time) = &rows[0];
    assert_eq!(*calls, 8);
    assert!(
        (*avgbytes - 96.0000).abs() < 1e-4,
        "avgbytes was {}, expected 96.0000",
        avgbytes
    );
    assert!(
        (*time - 2.193451e-05).abs() < 1e-4,
        "time was {}, expected 2.193451e-05",
        time
    );
    Ok(())
}

/// Function for testing import of GROMACS data in table iprof
/// by checking database contents after import.
///
/// # Arguments
/// - `pool`: reference to the database connection pool
///
/// # Returns
/// - `Result<()>`: Ok if all checks pass, Err otherwise
///
async fn check_gromacs_iprof_data(pool: &sqlx::Pool<MySql>) -> Result<()> {
    // Query the database
    let rows = sqlx::query_as::<_, (i32, u32)>(
        "SELECT `routine_id`, `ticks` FROM `iprof` WHERE `tid` = 0 AND routine_id < 4;",
    )
    .fetch_all(pool)
    .await?;

    // Assert exactly one row was returned
    assert_eq!(
        rows.len(),
        3,
        "Expected exactly 3 rows, but got {}",
        rows.len()
    );

    // Assert the values of the returned rows
    let expected_rows = vec![(1, 10725), (2, 568), (3, 489)];

    for expected in expected_rows {
        assert!(
            rows.contains(&expected),
            "Expected row {:?} not found in database",
            expected
        );
    }

    Ok(())
}
