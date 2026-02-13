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
    use crate::jobdata::table_iprof::extract_full_name;
    use anyhow::Result;
    use serde_yaml::Value;

    #[test]
    fn parses_single_string_element() -> Result<()> {
        let input = serde_yaml::from_str(r#"- "lib.so""#)?;
        let full_name = extract_full_name(&input)?;
        assert_eq!(full_name, "lib.so");
        Ok(())
    }

    #[test]
    fn joins_multiple_parts_in_order() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- "lib"
- "::"
- "func""#,
        )?;
        let full_name = extract_full_name(&input)?;
        assert_eq!(full_name, "lib::func");
        Ok(())
    }

    #[test]
    fn empty_sequence_returns_error() -> Result<()> {
        let input = serde_yaml::from_str(r#"[]"#)?;
        let err = extract_full_name(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a non-empty sequence"));
        Ok(())
    }

    #[test]
    fn rejects_non_sequence() -> Result<()> {
        let input = serde_yaml::from_str(r#""lib""#)?;
        let err = extract_full_name(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected a sequence"));
        Ok(())
    }

    #[test]
    fn rejects_non_string_member() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- "lib"
- 42"#,
        )?;
        let err = extract_full_name(&input).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Expected all members of the sequence to be strings"));
        Ok(())
    }

    #[test]
    fn preserves_exact_content() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- " a"
- "b ""#,
        )?;
        let full_name = extract_full_name(&input)?;
        assert_eq!(full_name, " ab ");
        Ok(())
    }

    #[test]
    fn handles_unicode() -> Result<()> {
        let input = serde_yaml::from_str(
            r#"- "µ"
- "λ""#,
        )?;
        let full_name = extract_full_name(&input)?;
        assert_eq!(full_name, "µλ");
        Ok(())
    }

    #[test]
    fn handles_large_sequence_without_panic() -> Result<()> {
        let parts: Vec<Value> = (0..10_000)
            .map(|_| Value::String("x".to_string()))
            .collect();
        let input = Value::Sequence(parts);
        let full_name = extract_full_name(&input)?;
        assert_eq!(full_name.len(), 10_000);
        Ok(())
    }
}
// ...existing code...
