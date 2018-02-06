use serde_json;

use std::fs::File;
use std::io::prelude::*;

pub fn load_json_file(path: &str) -> Result<serde_json::Value, String> {
    let contents = load_file(path)?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

fn load_file(path: &str) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    return Ok(contents)
}
