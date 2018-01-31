extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate hyper;

use std::error::Error;

mod parsers;
mod codacy;
mod source;

use source::Source;

// PUBLIC

// parse source with given options and send coverage over to codacy
pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  let default_type = "JSON";
  let file_type = config.value_of("TYPE").unwrap_or(default_type);

  let source = match file_type {
      "JSON" => Some(source::json_file::JsonFile::new(&config)),
      _ => None,
  };

  let input = source.unwrap().load()?;
  let parser: parsers::xcov::XCov = parsers::Parser::new(&input);

  codacy::report(&parser, &config)
}
