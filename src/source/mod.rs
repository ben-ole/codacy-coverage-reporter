use serde_json;

mod json_file;

pub enum SourceType {
    JsonFile
}

impl SourceType {

    pub fn new(from: &str) -> Result<SourceType, &'static str> {

        match from {
            "JSON" => Ok( SourceType::JsonFile ),
                _  => Err ( "Not supported parser" )
       } 
    }

    pub fn load(&self, path: &str) -> Result<serde_json::Value, String> {
  
        match *self {
            SourceType::JsonFile => json_file::load_json_file(path)
        }
    }

}
