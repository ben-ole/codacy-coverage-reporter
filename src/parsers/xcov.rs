use serde_json;
use std::collections::HashMap;

// XCov

pub fn total_cov_xcov(content: &serde_json::Value) -> u8 {
    let acc_cov = &content["coverage"].as_f64().unwrap() * 100.0;
    acc_cov.round() as u8
}

pub fn file_cov_xcov(content: &serde_json::Value, path_prefix: &str) -> serde_json::Value {
    let files = &content["files"].as_array().unwrap();

    let files_cov: Vec<serde_json::Value> = files.iter().map( |f| {

        let name = f["name"].as_str().unwrap();
        let filename = path_prefix.to_owned() + name;
        let total = f["coverage"].as_f64().unwrap() * 100.0;

        let function_level_coverage: HashMap<&str, u8> = HashMap::new();

        json!({
            "filename": filename,
            "total": total.round() as u8,
            "coverage": function_level_coverage
        })

    }).collect();

    json!(files_cov)
}
