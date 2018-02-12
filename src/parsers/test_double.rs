use serde_json;

pub fn total_cov() -> u8 {
    80
}

pub fn file_cov() -> serde_json::Value {

    let file = json!({
        "filename": "main.swift",
        "total": 80,
        "coverage": {
          "main": 80
        }
    });

    json!([file])
}
