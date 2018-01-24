extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate hyper;

mod parsers;
mod codacy;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// PUBLIC

// parse source with given options and send coverage over to codacy
pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  let source = read_source( config.value_of("INPUT").unwrap() )?;

  let parser: parsers::xcov::XCov = parsers::Parser::new(&source);

  codacy::report(&parser, config)
}

// PRIVATE

// load source coverage from file at given path and parse json
fn read_source(path: &str) -> Result<serde_json::Value, Box<Error>> {

  let mut f = File::open(path)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  Ok( serde_json::from_str(&contents)? )
}
