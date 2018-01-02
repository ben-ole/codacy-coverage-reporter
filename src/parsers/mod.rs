use serde_json;

pub mod xcov;

pub trait Parser<'a> {
  fn new(source: &'a serde_json::Value) -> Self;

  fn total_coverage(&self) -> u8;

  fn file_coverage(&self, path_prefix: &str) -> serde_json::Value;
}