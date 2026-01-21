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
mod tests {
    use crate::cmdline::CliArgs;
    use crate::jobdata::table_runs::foreign_keys::execute_query_if_pool;
    use anyhow::Result;
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_execute_query_if_pool(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: false,
            dry_run: false,
            do_import: true,
            ..Default::default()
        };
        let query = "SELECT 1 + 1 AS sum;";
        let result = execute_query_if_pool(&Some(pool.clone()), query, &args).await;
        assert!(result.is_ok(), "{}", result.as_ref().unwrap_err());
        Ok(())
    }

    #[sqlx::test(fixtures("../../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_execute_query_if_pool_dry_run(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: false,
            dry_run: true,
            do_import: true,
            ..Default::default()
        };
        let query = "SELECT 1 + 1 AS sum;";
        let result = execute_query_if_pool(&Some(pool.clone()), query, &args).await;
        assert!(result.is_ok(), "{}", result.as_ref().unwrap_err());
        Ok(())
    }

    #[tokio::test]
    pub async fn test_execute_query_if_pool_no_pool() -> Result<()> {
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: false,
            dry_run: false,
            do_import: true,
            ..Default::default()
        };
        let query = "SELECT 1 + 1 AS sum;";
        let result = execute_query_if_pool(&None, query, &args).await;
        assert!(result.is_ok(), "{}", result.as_ref().unwrap_err());
        Ok(())
    }

    #[sqlx::test(fixtures("../../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_execute_query_no_cluster_id(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: false,
            dry_run: false,
            do_import: false,
            ..Default::default()
        };
        let query = "SELECT cluster_id('Lenox', 0);";
        let result = execute_query_if_pool(&Some(pool.clone()), query, &args).await;
        assert!(
            result.is_err(),
            "Expected error due to missing cluster, got OK"
        );
        Ok(())
    }

    #[sqlx::test(fixtures("../../../../tests/fixtures/lmxtest.sql"))]
    pub async fn test_execute_query_cluster_id(pool: Pool<MySql>) -> Result<()> {
        let args = CliArgs {
            project_file: "not_there.yml".to_string(),
            verbose: false,
            dry_run: false,
            do_import: true,
            ..Default::default()
        };
        let query = "SELECT cluster_id('Lenox', 1);";
        let result = execute_query_if_pool(&Some(pool.clone()), query, &args).await;
        assert!(
            result.is_ok(),
            "Expected OK for existing cluster, got error: {}",
            result.as_ref().unwrap_err()
        );
        Ok(())
    }
}
