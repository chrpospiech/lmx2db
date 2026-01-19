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

use crate::sqltypes::SqlTypeHashMap;
use std::collections::HashMap;

pub fn check_sqltype_values(sqltype_map: &SqlTypeHashMap) -> bool {
    let compare_map: SqlTypeHashMap = HashMap::from([
        (
            "hpm_events".to_string(),
            HashMap::from([
                ("name".to_string(), "varchar(512)".to_string()),
                ("description".to_string(), "varchar(2048)".to_string()),
            ]),
        ),
        (
            "mpi_details".to_string(),
            HashMap::from([
                ("avgbytes".to_string(), "float".to_string()),
                ("calls".to_string(), "int(11)".to_string()),
            ]),
        ),
        (
            "power_types".to_string(),
            HashMap::from([(
                "aggregation".to_string(),
                "enum('current','average','maximum','integral')".to_string(),
            )]),
        ),
    ]);

    compare_map.iter().all(|(key, sub_map)| {
        sqltype_map.get(key).is_some_and(|existing_sub_map| {
            sub_map
                .iter()
                .all(|(sub_key, sub_value)| existing_sub_map.get(sub_key) == Some(sub_value))
        })
    })
}

pub fn check_sqltypes_file(sqltypes_file: String) -> bool {
    let contents =
        std::fs::read_to_string(sqltypes_file).expect("Failed to read sqltype file for checking");

    // basic content checks
    assert!(
        contents.contains("hpm_events:")
            && contents.contains("mpi_details:")
            && contents.contains("power_types:"),
        "SQL key file does not contain expected data"
    );

    // Check YAML structure (basic check)
    let yaml: serde_yml::Value =
        serde_yml::from_str(&contents).expect("Created sqltype file is not valid YAML");
    assert!(yaml.get("hpm_events").is_some());
    assert!(yaml.get("mpi_details").is_some());
    assert!(yaml.get("power_types").is_some());

    // Further checks can be added here to validate specific keys/values
    let sqltypes_map: HashMap<String, HashMap<String, String>> =
        serde_yml::from_str(&contents).expect("Failed to deserialize sqltype file into hashmap");
    check_sqltype_values(&sqltypes_map)
}
