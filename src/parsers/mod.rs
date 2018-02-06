use serde_json;
use std::collections::HashMap;

pub enum Parser {
    Xcov(serde_json::Value)
}

impl Parser {

    pub fn new(from: &str, data: serde_json::Value) -> Result<Parser, &'static str> {

        match from {
            "XCOV" => Ok( Parser::Xcov(data["targets"][0].to_owned()) ),
                _  => Err ( "Not supported parser" )
        }
    }

    pub fn total_coverage(&self) -> u8 {

        match *self {
            Parser::Xcov(ref content) => self.total_cov_xcov(content)
        }
    }

    pub fn file_coverage(&self, path_prefix: &str) -> serde_json::Value  {

        match *self {
            Parser::Xcov(ref content) => self.file_cov_xcov(&content, path_prefix)
        }
    }

    // XCov

    fn total_cov_xcov(&self, content: &serde_json::Value) -> u8 {        
        let acc_cov = &content["coverage"].as_f64().unwrap() * 100.0;
        acc_cov.round() as u8
    }

    fn file_cov_xcov(&self, content: &serde_json::Value, path_prefix: &str) -> serde_json::Value {        
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
}
