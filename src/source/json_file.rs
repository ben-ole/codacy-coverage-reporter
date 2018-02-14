use serde_json;

use std::fs::File;
use std::io::prelude::*;

pub fn load(path: &str) -> Result<serde_json::Value, String> {
    let contents = load_file(path)?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

fn load_file(path: &str) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    return Ok(contents)
}

// UNIT TESTS

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::env;
    
    #[test]
    fn test_load_file() {

        let expected_content = "{\"codacy\": \"🚀\"}";
        create_test_file("dummy.json", expected_content);

        let mut dir = env::temp_dir();
        dir.push("dummy.json");
        let actual_content = load_file(dir.to_str().unwrap());

        assert_eq!(actual_content.unwrap(), expected_content);
    }


    #[test]
    fn test_load() {

        let file_content = "{\"codacy\": \"🚀\"}";
        create_test_file("dummy.json", file_content);

        let mut dir = env::temp_dir();
        dir.push("dummy.json");
        let actual_content = load(dir.to_str().unwrap());
        assert_eq!(actual_content.unwrap()["codacy"], "🚀");
    }

    // test utils

    pub fn create_test_file(name: &str, text: &str) -> String {
        
        let mut dir = env::temp_dir();
        dir.push(name);
        
        let mut f = File::create(dir.to_owned()).unwrap();
        f.write_all(text.as_bytes()).unwrap();
        f.sync_all().unwrap();

        dir.to_str().unwrap().to_owned()
    }
}
