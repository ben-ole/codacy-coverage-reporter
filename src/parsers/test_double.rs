use serde_json;
use std::collections::HashMap;

pub fn total_cov() -> u8 {
    80
}

pub fn file_cov() -> serde_json::Value {
    
    let function_level_coverage: HashMap<&str, u8> = HashMap::new();

    let file = json!({
        "filename": "main.swift",
        "total": 80,
        "coverage": function_level_coverage
    });

    json!([file])
}
