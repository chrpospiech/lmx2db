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

use crate::jobdata::LmxSummary;
use anyhow::{anyhow, bail, Result};

/// Extracts MPI rank information from an LMX summary type structure.
///
/// # Arguments
/// * `lmx_summary` - A reference to the LMX summary data structure.
///
/// # Returns
/// * `Result<u64>` - The extracted MPI rank as a u64 integer, or an error if extraction fails.
#[allow(dead_code)]
pub fn extract_mpi_rank(lmx_summary: &LmxSummary) -> Result<u64> {
    if let Some(base_data) = lmx_summary.get("base_data") {
        if let Some(mpi_rank_value) = base_data.get("MPI_ranks") {
            // First, try to interpret the value as an unsigned integer directly.
            if let Some(mpi_rank) = mpi_rank_value.as_u64() {
                Ok(mpi_rank)
            } else if let Some(mpi_rank_i) = mpi_rank_value.as_i64() {
                if mpi_rank_i < 0 {
                    bail!(
                        "MPI_ranks value in base_data is negative and cannot be converted to u64"
                    );
                } else {
                    Ok(mpi_rank_i as u64)
                }
            } else if let Some(mpi_rank_str) = mpi_rank_value.as_str() {
                let mpi_rank = mpi_rank_str.parse::<u64>().map_err(|e| {
                    anyhow!(
                        "Failed to parse MPI_ranks value '{}' in base_data as u64: {}",
                        mpi_rank_str,
                        e
                    )
                })?;
                Ok(mpi_rank)
            } else {
                bail!("MPI_ranks value in base_data is neither a number nor a string");
            }
        } else {
            bail!("MPI_ranks key not found in base_data");
        }
    } else {
        bail!("base_data key not found in LMX summary");
    }
}
