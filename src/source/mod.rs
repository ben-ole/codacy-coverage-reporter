use serde_json;

use std::fs::File;
use std::io::prelude::*;

pub enum SourceType {
    JsonFile
}

impl SourceType {

    pub fn new(from: &str) -> Result<SourceType, &'static str> {

        match from {
            "JSON" => Ok( SourceType::JsonFile ),
            "XML"  => Err( "Not yet supported" ),
                _  => Ok ( SourceType::JsonFile ) // default
       } 
    }

    pub fn load(&self, path: &str) -> Result<serde_json::Value, String> {
  
        match *self {
            SourceType::JsonFile => self.load_json_file(path),
        }
    }

    fn load_json_file(&self, path: &str) -> Result<serde_json::Value, String> {
        let contents = self.load_file(path)?;
        serde_json::from_str(&contents).map_err(|e| e.to_string())
    }

    fn load_file(&self, path: &str) -> Result<String, String> {
        let mut f = File::open(path).map_err(|e| e.to_string())?;

        let mut contents = String::new();
        f.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        return Ok(contents)
    }
}
