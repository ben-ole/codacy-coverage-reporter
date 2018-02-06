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
mod tests {

    use super::*;
    use std::fs::*;

    #[test]
    fn test_load_file() {

        let expected_content = "{\"codacy\": \"🚀\"}";
        create_test_file("tmp/dummy.json", expected_content);

        let actual_content = load_file("tmp/dummy.json");

        assert_eq!(actual_content.unwrap(), expected_content);
    }


    #[test]
    fn test_load() {

        let file_content = "{\"codacy\": \"🚀\"}";
        create_test_file("tmp/dummy.json", file_content);

        let actual_content = load("tmp/dummy.json");
        assert_eq!(actual_content.unwrap()["codacy"], "🚀");
    }

    // test utils

    fn create_test_file(name: &str, text: &str) {
        create_dir("tmp/").unwrap_or(());
        let mut f = File::create(name).unwrap();
        f.write_all(text.as_bytes()).unwrap();
        f.sync_all().unwrap();
    }
}
