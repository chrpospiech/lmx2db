use crate::jobdata::LmxSummary;
use anyhow::Result;

#[cfg(test)]
pub(crate) mod collect_time;
#[cfg(test)]
pub(crate) mod elapsed_time;

/// Computes the collection time duration from LMX summary data.
///
/// This function calculates the total collection time by combining:
/// - The difference between `stop_date` and `start_date` (seconds component)
/// - The difference between `stop_date_n` and `start_date_n` (nanoseconds component, converted to seconds)
///
/// # Arguments
///
/// * `lmx_summary` - A reference to the LMX summary data containing timing information
///
/// # Returns
///
/// * `Result<serde_yaml::Value>` - The computed collection time as a YAML value on success
///
/// # Errors
///
/// This function will return an error if:
/// * The `base_data` section is not found in the LMX summary
/// * Any required field (`start_date`, `stop_date`, `start_date_n`, `stop_date_n`) is missing
/// * Any of the date fields cannot be converted to a valid `f64` number
/// * The computed value cannot be serialized to a YAML value
///
/// # Formula
///
/// ```text
/// collect_time = (stop_date - start_date) + (stop_date_n - start_date_n) × 10⁻⁹
/// ```
///
/// # Example
///
/// ```no_run
/// # use lmx2db::jobdata::LmxSummary;
/// # use lmx2db::jobdata::table_runs::timing_data::compute_collect_time;
/// let lmx_summary: LmxSummary = /* ... */;
/// let collect_time = compute_collect_time(&lmx_summary)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn compute_collect_time(lmx_summary: &LmxSummary) -> Result<serde_yaml::Value> {
    if let Some(base_data) = lmx_summary.get("base_data") {
        // Extract the required fields from base_data
        let start_date = base_data
            .get("start_date")
            .ok_or_else(|| anyhow::anyhow!("'start_date' not found in 'base_data'"))?;
        let stop_date = base_data
            .get("stop_date")
            .ok_or_else(|| anyhow::anyhow!("'stop_date' not found in 'base_data'"))?;
        let start_date_n = base_data
            .get("start_date_n")
            .ok_or_else(|| anyhow::anyhow!("'start_date_n' not found in 'base_data'"))?;
        let stop_date_n = base_data
            .get("stop_date_n")
            .ok_or_else(|| anyhow::anyhow!("'stop_date_n' not found in 'base_data'"))?;
        // Try to convert these numbers to f64
        let start_float = match start_date {
            serde_yaml::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("Invalid 'start_date' value"))?,
            _ => return Err(anyhow::anyhow!("'start_date' is not a number")),
        };
        let stop_float = match stop_date {
            serde_yaml::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("Invalid 'stop_date' value"))?,
            _ => return Err(anyhow::anyhow!("'stop_date' is not a number")),
        };
        let start_n_float = match start_date_n {
            serde_yaml::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("Invalid 'start_date_n' value"))?,
            _ => return Err(anyhow::anyhow!("'start_date_n' is not a number")),
        };
        let stop_n_float = match stop_date_n {
            serde_yaml::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("Invalid 'stop_date_n' value"))?,
            _ => return Err(anyhow::anyhow!("'stop_date_n' is not a number")),
        };
        // Compute collect_time as per the formula
        let collect_time =
            (stop_float - start_float) + (stop_n_float - start_n_float) * 0.000000001;
        Ok(serde_yaml::to_value(collect_time)?)
    } else {
        Err(anyhow::anyhow!(
            "'base_data' section not found in LmxSummary"
        ))
    }
}

/// Computes the elapsed time duration from LMX summary data.
///
/// This function calculates the total elapsed time by taking the maximum of
/// the elapsed times reported in the rank_summary section in the LMX summary.
/// The rank_summary section is a HashMap<String, Vec<float>>, where each key
/// is a rank identifier. The elapsed time for each rank is the first element
/// in the corresponding vector.
///
/// # Arguments
///
/// * `lmx_summary` - A reference to the LMX summary data containing timing information
///
/// # Returns
///
/// * `Result<serde_yaml::Value>` - The computed elapsed time as a YAML value on success
///
/// # Errors
///
/// This function will return an error if:
/// * The `rank_summary` section is not found in the LMX summary
/// * Any required field is missing
/// * Any of the float fields cannot be converted to a valid `f64` number
/// * The computed value cannot be serialized to a YAML value
///
/// # Formula
///```text
/// elapsed_time = max(rank_summary[rank][0] for each rank)
///```
/// # Example
///```no_run
/// # use lmx2db::jobdata::LmxSummary;
/// # use lmx2db::jobdata::table_runs::timing_data::compute_elapsed_time;
/// let lmx_summary: LmxSummary = /* ... */;
/// let elapsed_time = compute_elapsed_time(&lmx_summary)?;
/// # Ok::<(), anyhow::Error>(())
///```
pub fn compute_elapsed_time(lmx_summary: &LmxSummary) -> Result<serde_yaml::Value> {
    if let Some(rank_summary) = lmx_summary.get("rank_summary") {
        let mut max_elapsed_time: Option<f64> = None;
        for values in rank_summary.values() {
            let value_vec = values
                .as_sequence()
                .ok_or_else(|| anyhow::anyhow!("Rank values are not a sequence"))?;
            if let Some(first_value) = value_vec.first() {
                let elapsed_time = match first_value {
                    serde_yaml::Value::Number(n) => n
                        .as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid elapsed time value"))?,
                    _ => return Err(anyhow::anyhow!("Elapsed time is not a number")),
                };
                max_elapsed_time = match max_elapsed_time {
                    Some(current_max) => Some(current_max.max(elapsed_time)),
                    None => Some(elapsed_time),
                };
            }
        }
        if let Some(max_time) = max_elapsed_time {
            Ok(serde_yaml::to_value(max_time)?)
        } else {
            Err(anyhow::anyhow!(
                "No elapsed time values found in 'rank_summary'"
            ))
        }
    } else {
        Err(anyhow::anyhow!(
            "'rank_summary' section not found in LmxSummary"
        ))
    }
}
