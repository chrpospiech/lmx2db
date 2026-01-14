#[cfg(test)]
mod tests {
    use crate::jobdata::table_runs::timing_data::compute_collect_time;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;
    use serde_yaml::Value;

    #[test]
    fn test_compute_collect_time() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let collect_time = compute_collect_time(&lmx_summary)?;
        if let Value::Number(num) = collect_time {
            let ct = num.as_f64().unwrap();
            // Expected collect_time = (1622552400 - 1622548800) + (800000000 - 500000000) * 0.000000001
            let expected = 3600.3;
            assert!(
                (ct - expected).abs() < 1e-6,
                "collect_time calculation is incorrect"
            );
        } else {
            panic!("collect_time is not a number");
        }
        Ok(())
    }

    #[test]
    fn test_wrong_lmx_summary() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
basic_data:
  start_date: 1622548800
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = compute_collect_time(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "'base_data' section not found in LmxSummary"
        );
        Ok(())
    }

    #[test]
    fn test_no_start_date() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
base_data:
  stop_date: 1622552400
  start_date_n: 500000000
  stop_date_n: 800000000
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = compute_collect_time(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "'start_date' not found in 'base_data'"
        );
        Ok(())
    }
}
