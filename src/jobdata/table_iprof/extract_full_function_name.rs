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
    use crate::jobdata::table_iprof::extract_full_function_name;
    use crate::jobdata::LmxSummary;
    use anyhow::Result;

    #[test]
    fn parses_single_string_element() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    myfunc:
    - "do_work""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let full_name = extract_full_function_name(&iprof, &lib, &func)?;
        assert_eq!(full_name, "do_work");
        Ok(())
    }

    #[test]
    fn joins_multiple_parts_in_order() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    myfunc:
    - 'Namespace'
    - '::'
    - 'do_work'"#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let full_name = extract_full_function_name(&iprof, &lib, &func)?;
        assert_eq!(full_name, "Namespace::do_work");
        Ok(())
    }

    #[test]
    fn handles_empty_function_name() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    myfunc: []"#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected a non-empty sequence"));
        Ok(())
    }

    #[test]
    fn rejects_non_sequence_function_value() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    myfunc: "do_work""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected a sequence"));
        Ok(())
    }

    #[test]
    fn bails_on_missing_subroutine_names_section() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"other_section:
  mylib:
    myfunc:
    - "do_work""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("'subroutine_names' section not found"));
        Ok(())
    }

    #[test]
    fn bails_on_missing_library_in_subroutine_names() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  otherlib:
    myfunc:
    - "do_work""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Library name 'mylib' not found in 'subroutine_names'"));
        Ok(())
    }

    #[test]
    fn bails_on_non_mapping_library_value() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib: "not_a_mapping""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected a mapping"));
        Ok(())
    }

    #[test]
    fn bails_on_missing_short_name() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    otherfunc:
    - "do_work""#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Short name 'myfunc' not found in 'subroutine_names'"));
        Ok(())
    }

    #[test]
    fn rejects_non_string_sequence_member() -> Result<()> {
        let iprof: LmxSummary = serde_yaml::from_str(
            r#"subroutine_names:
  mylib:
    myfunc:
    - 42"#,
        )?;
        let lib = "mylib".to_string();
        let func = "myfunc".to_string();
        let result = extract_full_function_name(&iprof, &lib, &func);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Expected all members"));
        Ok(())
    }
}
