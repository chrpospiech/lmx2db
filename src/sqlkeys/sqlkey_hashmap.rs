use std::collections::HashMap;

pub fn check_sqlkey_values(sqlkey_map: &HashMap<String, HashMap<String, String>>) -> bool {
    let compare_map: HashMap<String, HashMap<String, String>> = HashMap::from([
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
        sqlkey_map.get(key).is_some_and(|existing_sub_map| {
            sub_map
                .iter()
                .all(|(sub_key, sub_value)| existing_sub_map.get(sub_key) == Some(sub_value))
        })
    })
}

#[allow(dead_code)]
pub fn check_sqlkeys_file(sqlkeys_file: String) -> bool {
    let contents =
        std::fs::read_to_string(sqlkeys_file).expect("Failed to read sqlkey file for checking");

    // basic content checks
    assert!(
        contents.contains("hpm_events:")
            && contents.contains("mpi_details:")
            && contents.contains("power_types:"),
        "SQL key file does not contain expected data"
    );

    // Check YAML structure (basic check)
    let yaml: serde_yml::Value =
        serde_yml::from_str(&contents).expect("Created sqlkey file is not valid YAML");
    assert!(yaml.get("hpm_events").is_some());
    assert!(yaml.get("mpi_details").is_some());
    assert!(yaml.get("power_types").is_some());

    // Further checks can be added here to validate specific keys/values
    let sqlkeys_map: HashMap<String, HashMap<String, String>> =
        serde_yml::from_str(&contents).expect("Failed to deserialize sqlkey file into hashmap");
    check_sqlkey_values(&sqlkeys_map)
}
