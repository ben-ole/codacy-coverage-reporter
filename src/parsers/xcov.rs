use serde_json;
use std::collections::HashMap;

// internal
use parsers::Parser;

#[derive(Debug)]
pub struct XCov<'a> {
    source: &'a serde_json::Value
}

impl<'a> Parser<'a> for XCov<'a> {

    fn new(source: &'a serde_json::Value) -> XCov {
      let first_target = &source["targets"][0];
      XCov { source: first_target }
    }

    fn total_coverage(&self) -> u8 {
      let acc_cov = self.source["coverage"].as_f64().unwrap() * 100.0;
      acc_cov.round() as u8
    }

    fn file_coverage(&self, path_prefix: &str) -> serde_json::Value {
      let files = self.source["files"].as_array().unwrap();

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

}