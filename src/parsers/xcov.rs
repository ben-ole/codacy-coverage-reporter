use serde_json;
use std::collections::HashMap;

pub fn total_cov(content: &serde_json::Value) -> u8 {
    let acc_cov = &content["coverage"].as_f64().unwrap() * 100.0;
    acc_cov.round() as u8
}

pub fn file_cov(content: &serde_json::Value, path_prefix: &str) -> serde_json::Value {
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

// UNIT TESTS

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_total_cov() {
        let actual_value = total_cov( &sample_source() );

        assert_eq!(actual_value, 81);
    }

    #[test]
    fn test_file_cov() {
        let actual_value = file_cov( &sample_source(), "app/" );

        assert_eq!(actual_value[0]["filename"], "app/One.swift");
        assert_eq!(actual_value[0]["total"], 80);

        assert_eq!(actual_value[1]["filename"], "app/Two.swift");
        assert_eq!(actual_value[1]["total"], 0);
    }

    fn sample_source() -> serde_json::Value {
        json!({
            "name": "Sevenmind.app",
            "coverage": 0.8123456,
            "files": [
            {
              "name": "One.swift",
              "coverage": 0.8012443,
              "type": "swift",
              "functions": [
                {
                  "name": "App.One.methodOne(arg:String)-&gt;()",
                  "coverage": 0.7
                },
                {
                  "name": "App.One.methodTwo(arg:String)-&gt;()",
                  "coverage": 0.9
                }
              ]
            },
            {
              "name": "Two.swift",
              "coverage": 0,
              "type": "swift",
              "functions": [
                {
                  "name": "App.Two.methodOne(arg:String)-&gt;()",
                  "coverage": 0
                }
              ]
            }
            ]
        })
    }

}
