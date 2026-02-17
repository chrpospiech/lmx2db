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
    use crate::jobdata::table_iprof::extract_iprof_ticks;
    use anyhow::Result;

    #[test]
    fn parses_correct_tick_record() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- 100
"#,
        )?;
        let ticks = extract_iprof_ticks(&input)?;
        assert_eq!(ticks, 100);
        Ok(())
    }

    #[test]
    fn rejects_non_sequence() -> Result<()> {
        let input = serde_yaml::from_str(r#"100"#)?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a sequence"));
        Ok(())
    }

    #[test]
    fn rejects_non_integer_first_value() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- "not an integer"
"#,
        )?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a sequence with an integer"));
        Ok(())
    }

    #[test]
    fn empty_sequence_returns_error() -> Result<()> {
        let input = serde_yaml::from_str(r#"[]"#)?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a sequence with an integer"));
        Ok(())
    }

    #[test]
    fn uses_only_first_element_from_multiple() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- 100
- 200
- 300
"#,
        )?;
        let ticks = extract_iprof_ticks(&input)?;
        assert_eq!(ticks, 100);
        Ok(())
    }

    #[test]
    fn rejects_negative_tick_value() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- -100
"#,
        )?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a sequence with an integer"));
        Ok(())
    }

    #[test]
    fn rejects_i64_value_above_i32_max() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- 3000000000
"#,
        )?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Interval timer profiler ticks value"));
        assert!(msg.contains("is out of u64 range"));
        assert!(msg.contains("3000000000"));
        Ok(())
    }

    #[test]
    fn rejects_i64_value_below_i32_min() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- -3000000000
"#,
        )?;
        let err = extract_iprof_ticks(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Interval timer profiler ticks value"));
        assert!(msg.contains("is out of i32 range"));
        assert!(msg.contains("-3000000000"));
        Ok(())
    }
}
