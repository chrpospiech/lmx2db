#[cfg(test)]
mod tests {
    use crate::jobdata::table_runs::timing_data::compute_elapsed_time;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;
    use serde_yaml::Value;

    #[test]
    fn test_compute_elapsed_time() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
rank_summary:
  rank_0: [3600.5, 12345]
  rank_1: [3599.8, 67890]
  rank_2: [3601.2, 54321]
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let elapsed_time = compute_elapsed_time(&lmx_summary)?;
        if let Value::Number(num) = elapsed_time {
            let et = num.as_f64().unwrap();
            let expected = 3601.2; // The maximum elapsed time among ranks
            assert!(
                (et - expected).abs() < 1e-6,
                "elapsed_time calculation is incorrect"
            );
        } else {
            panic!("elapsed_time is not a number");
        }
        Ok(())
    }

    #[test]
    fn test_no_rank_summary() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
basic_data:
  some_field: 12345
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = compute_elapsed_time(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "'rank_summary' section not found in LmxSummary"
        );
        Ok(())
    }

    #[test]
    fn test_rank_summary_wrong_type() -> Result<()> {
        // Sample LmxSummary data for testing
        let yaml_data = r#"
rank_summary:
  rank_0: 3600.5
  rank_1: 3599.8
  rank_2: 3601.2
"#;
        let lmx_summary: LmxSummary = serde_yaml::from_str(yaml_data)?;
        let result = compute_elapsed_time(&lmx_summary);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Rank values are not a sequence"
        );
        Ok(())
    }
}
