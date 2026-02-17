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
    use crate::jobdata::checktypes::check_types;
    use anyhow::Result;

    #[test]
    fn test_bigint_unsigned_lowercase() -> Result<()> {
        // Test that lowercase "bigint(20) unsigned" is properly handled
        let keys = vec!["count".to_string()];
        let types = vec!["bigint(20) unsigned".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u64::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u64 value for unsigned bigint");
        Ok(())
    }

    #[test]
    fn test_bigint_signed_lowercase() -> Result<()> {
        // Test that lowercase "bigint(20)" is properly handled
        let keys = vec!["count".to_string()];
        let types = vec!["bigint(20)".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            i64::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max i64 value for signed bigint");
        Ok(())
    }

    #[test]
    fn test_bigint_signed_negative() -> Result<()> {
        // Test that lowercase "bigint(20)" handles negative values
        let keys = vec!["count".to_string()];
        let types = vec!["bigint(20)".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            i64::MIN,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept min i64 value for signed bigint");
        Ok(())
    }

    #[test]
    fn test_int_unsigned_lowercase() -> Result<()> {
        // Test that lowercase "int(11) unsigned" is properly handled
        let keys = vec!["tid".to_string()];
        let types = vec!["int(11) unsigned".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u32::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u32 value for unsigned int");
        Ok(())
    }

    #[test]
    fn test_int_unsigned_overflow() -> Result<()> {
        // Test that unsigned int rejects values > u32::MAX
        let keys = vec!["tid".to_string()];
        let types = vec!["int(11) unsigned".to_string()];
        let too_large: u64 = u32::MAX as u64 + 1;
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            too_large,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_err(), "Should reject value > u32::MAX for unsigned int");
        assert!(
            result.unwrap_err().to_string().contains("u32 range"),
            "Error should mention u32 range"
        );
        Ok(())
    }

    #[test]
    fn test_int_signed_lowercase() -> Result<()> {
        // Test that lowercase "int(11)" is properly handled
        let keys = vec!["calls".to_string()];
        let types = vec!["int(11)".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            i32::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max i32 value for signed int");
        Ok(())
    }

    #[test]
    fn test_int_signed_overflow() -> Result<()> {
        // Test that signed int rejects values > i32::MAX
        let keys = vec!["calls".to_string()];
        let types = vec!["int(11)".to_string()];
        let too_large: i64 = i32::MAX as i64 + 1;
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            too_large,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_err(), "Should reject value > i32::MAX for signed int");
        assert!(
            result.unwrap_err().to_string().contains("i32 range"),
            "Error should mention i32 range"
        );
        Ok(())
    }

    #[test]
    fn test_tinyint_unsigned_lowercase() -> Result<()> {
        // Test that lowercase "tinyint(4) unsigned" is properly handled
        let keys = vec!["ht".to_string()];
        let types = vec!["tinyint(4) unsigned".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u8::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u8 value for unsigned tinyint");
        Ok(())
    }

    #[test]
    fn test_tinyint_unsigned_overflow() -> Result<()> {
        // Test that unsigned tinyint rejects values > u8::MAX
        let keys = vec!["ht".to_string()];
        let types = vec!["tinyint(4) unsigned".to_string()];
        let too_large: u16 = u8::MAX as u16 + 1;
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            too_large,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_err(), "Should reject value > u8::MAX for unsigned tinyint");
        assert!(
            result.unwrap_err().to_string().contains("tinyint range"),
            "Error should mention tinyint range"
        );
        Ok(())
    }

    #[test]
    fn test_smallint_unsigned_lowercase() -> Result<()> {
        // Test that lowercase "smallint(6) unsigned" is properly handled
        let keys = vec!["regid".to_string()];
        let types = vec!["smallint(6) unsigned".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u16::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u16 value for unsigned smallint");
        Ok(())
    }

    #[test]
    fn test_smallint_unsigned_overflow() -> Result<()> {
        // Test that unsigned smallint rejects values > u16::MAX
        let keys = vec!["regid".to_string()];
        let types = vec!["smallint(6) unsigned".to_string()];
        let too_large: u32 = u16::MAX as u32 + 1;
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            too_large,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_err(), "Should reject value > u16::MAX for unsigned smallint");
        assert!(
            result.unwrap_err().to_string().contains("smallint range"),
            "Error should mention smallint range"
        );
        Ok(())
    }

    #[test]
    fn test_case_insensitive_uppercase_bigint() -> Result<()> {
        // Test that uppercase "BIGINT(20) UNSIGNED" is also handled (case-insensitive)
        let keys = vec!["count".to_string()];
        let types = vec!["BIGINT(20) UNSIGNED".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u64::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u64 value for BIGINT UNSIGNED (uppercase)");
        Ok(())
    }

    #[test]
    fn test_mixed_case_int() -> Result<()> {
        // Test that mixed case "Int(11) Unsigned" is also handled
        let keys = vec!["tid".to_string()];
        let types = vec!["Int(11) Unsigned".to_string()];
        let values = vec![vec![serde_yaml::Value::Number(serde_yaml::Number::from(
            u32::MAX,
        ))]];
        let result = check_types("test_table", &keys, &types, &values);
        assert!(result.is_ok(), "Should accept max u32 value for Int Unsigned (mixed case)");
        Ok(())
    }
}
