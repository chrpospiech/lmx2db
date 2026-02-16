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
    use crate::jobdata::table_iprof::extract_full_library_name;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;

    #[test]
    fn parses_single_string_element() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"library_names:
  library_name:
  - "lib.so""#,
        )?;
        let short_name = "library_name".to_string();
        let full_name = extract_full_library_name(&iprof, &short_name)?;
        assert_eq!(full_name, "lib.so");
        Ok(())
    }

    #[test]
    fn joins_multiple_parts_in_order() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"library_names:
  library_name:
  - 'lib'
  - '::'
  - 'func'"#,
        )?;
        let short_name = "library_name".to_string();
        let full_name = extract_full_library_name(&iprof, &short_name)?;
        assert_eq!(full_name, "lib::func");
        Ok(())
    }

    #[test]
    fn handles_empty_library_name() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"library_names:
  library_name: []"#,
        )?;
        let short_name = "library_name".to_string();
        let result = extract_full_library_name(&iprof, &short_name);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected a non-empty sequence"));
        Ok(())
    }

    #[test]
    fn rejects_non_sequence_library_name() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"library_names:
  library_name: "lib.so""#,
        )?;
        let short_name = "library_name".to_string();
        let result = extract_full_library_name(&iprof, &short_name);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected a sequence"));
        Ok(())
    }

    #[test]
    fn bails_out_on_missing_short_name() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"library_names:
  other_library:
  - "lib.so""#,
        )?;
        let short_name = "library_name".to_string();
        let result = extract_full_library_name(&iprof, &short_name);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Short name 'library_name' not found in 'library_names'"));
        Ok(())
    }
}
