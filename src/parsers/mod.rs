use serde_json;

pub mod xcov;

// generic trait interface for parsers 
pub trait Parser<'a> {
  fn new(source: &'a serde_json::Value) -> Self where Self: Sized;

  fn total_coverage(&self) -> u8;

  fn file_coverage(&self, path_prefix: &str) -> serde_json::Value;
}
