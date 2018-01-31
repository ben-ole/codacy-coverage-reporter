use serde_json;
use clap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// internal
use source::Source;

pub struct JsonFile<'a> {
    path: &'a str
}

// implementation of the generic source trait for `json file` source
impl<'a> Source<'a> for JsonFile<'a> {

    fn new(config: &'a clap::ArgMatches) -> JsonFile<'a> {
        let path = &config.value_of("INPUT").unwrap();
        JsonFile { path: path }
    }

    fn load(&self) -> Result<serde_json::Value, Box<Error>> {
        
        let mut f = File::open(self.path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        Ok( serde_json::from_str(&contents)? )
    }
}
