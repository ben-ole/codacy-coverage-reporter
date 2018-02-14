use serde_json;

mod json_file;

pub enum SourceType {
    JsonFile
}

impl SourceType {

    // convenience initializer: load SourceType from string
    pub fn new(from: &str) -> Result<SourceType, &'static str> {

        match from {
            "JSON" => Ok( SourceType::JsonFile ),
                _  => Err ( "Not supported parser" )
       } 
    }

    // load content of source into serde_json::value struct
    pub fn load(&self, path: &str) -> Result<serde_json::Value, String> {
  
        match *self {
            SourceType::JsonFile => json_file::load(path)
        }
    }

}

// UNIT TESTS

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_json() {
        let actual_result = SourceType::new("JSON");

        assert!(actual_result.is_ok());        
        assert!(matches!(actual_result.unwrap(), SourceType::JsonFile));        
    }

    #[test]
    fn test_new_bad() {
        let actual_result = SourceType::new("BAD");

        assert!(actual_result.is_err());
    }

    #[test]
    fn test_load() {
        let file_content = "{\"codacy\": \"🚀\"}";
        let path = json_file::tests::create_test_file("dummy.json", file_content);

        let source = SourceType::new("JSON").unwrap();

        let actual_content = source.load(&path);
        assert_eq!(actual_content.unwrap()["codacy"], "🚀");
    }
}
