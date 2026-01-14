use crate::jobdata::LmxSummary;
use anyhow::Result;

#[cfg(test)]
pub(crate) mod collect_time;

/// Function to compute the value of column `collect_time` in table `runs`
/// based on the provided LmxSummary data.
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
