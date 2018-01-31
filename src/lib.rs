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

  let source: source::json_file::JsonFile = source::Source::new(&config);
  let json = source.load()?;
    
  let parser: parsers::xcov::XCov = parsers::Parser::new(&json);

  codacy::report(&parser, &config)
}
