extern crate clap;

#[macro_use]
extern crate serde_json;

extern crate reqwest;

#[macro_use]
extern crate hyper;

mod parsers;
mod http;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

use parsers::Parser;

pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  let source = read_source( config.value_of("INPUT").unwrap() )?;

  let path_prefix = config.value_of("PREFIX").unwrap_or("");
  let parser: parsers::xcov::XCov = parsers::Parser::new(&source);

  let body = json!({
    "total": parser.total_coverage(),
    "fileReports": parser.file_coverage(path_prefix)
  });

  if let Some(ref path) = config.value_of("OUTPUT") {
    let mut file = File::create(path)?;
    file.write_all(body.to_string().as_bytes())?;
  }

  if config.occurrences_of("v") == 2 {
    println!("{}", body);
  }

  http::send( body.to_string(), 
              config.value_of("COMMIT").unwrap(),
              config.value_of("LANGUAGE").unwrap_or("swift"),
              config.value_of("PROJECT_TOKEN").unwrap() )
}

// Helper

fn read_source(path: &str) -> Result<serde_json::Value, Box<Error>> {

  let mut f = File::open(path)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  Ok( serde_json::from_str(&contents)? )
}
