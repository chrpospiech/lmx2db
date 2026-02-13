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

#[cfg(test)]
pub(crate) mod extract_full_name;
#[cfg(test)]
pub(crate) mod extract_iprof_ticks;

/// Helper function to extract the number of interval timer profiler ticks.
/// This function takes a reference to a single serde_yaml::Value that is
/// expected to be a sequence (array), checks that the first value is
/// an integer (i32), and returns that integer as the number of ticks.
///
/// Arguments:
/// * `value` - A reference to a serde_yaml::Value that should be a sequence
///
/// Returns the number of interval timer profiler ticks as an i32.
/// If the input value is not a sequence or if the first value
/// is not an integer, an error is returned.
#[allow(dead_code)]
pub fn extract_iprof_ticks(value: &serde_yaml::Value) -> Result<i32> {
    if let Some(seq) = value.as_sequence() {
        if let Some(first_value) = seq.first() {
            if let Some(ticks) = first_value.as_i64() {
                if ticks < i32::MIN as i64 || ticks > i32::MAX as i64 {
                    anyhow::bail!(
                        "Interval timer profiler ticks value {} is out of i32 range ({}..={})",
                        ticks,
                        i32::MIN,
                        i32::MAX
                    );
                }
                return Ok(ticks as i32);
            }
        }
    }
    anyhow::bail!(
        "Expected a sequence with an integer for interval timer profiler ticks, but got: {:?}",
        value
    );
}

/// Helper function to extract a full library or function name.
/// This function takes a reference to a single serde_yaml::Value that is
/// expected to be a sequence (array), checks that all members are strings,
/// and returns the joined string as the full name.
///
/// Arguments:
/// * `value` - A reference to a serde_yaml::Value that should be a sequence
///
/// Returns the full library or function name as a String.
/// If the input value is not a sequence or if any member is not a string,
/// an error is returned.
#[allow(dead_code)]
pub fn extract_full_name(value: &serde_yaml::Value) -> Result<String> {
    if let Some(seq) = value.as_sequence() {
        let mut full_name_parts = Vec::new();
        for member in seq {
            if let Some(part) = member.as_str() {
                full_name_parts.push(part);
            } else {
                anyhow::bail!(
                    "Expected all members of the sequence to be strings, but got: {:?}",
                    member
                );
            }
        }
        let full_name = full_name_parts.join("");
        if full_name.is_empty() {
            anyhow::bail!(
                "Expected a non-empty sequence for full name extraction, but got an empty sequence"
            );
        }
        return Ok(full_name);
    }
    anyhow::bail!(
        "Expected a sequence for full name extraction, but got: {:?}",
        value
    );
}
