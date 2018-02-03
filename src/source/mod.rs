use serde_json;
use std::error::Error;
use clap;

pub mod json_file;
pub mod xml_file;

// generic trait interface for source loader 
pub trait Source<'a> {
  fn new(config: &'a clap::ArgMatches) -> Self where Self: Sized;

  fn load(&self) -> Result<serde_json::Value, Box<Error>>;
}
