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

// use parsers::Parser;

// PUBLIC

// parse source with given options and send coverage over to codacy
pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  // select source loader based on arguments
  let file_type = config.value_of("TYPE").unwrap_or("");
  let source = source::SourceType::new(file_type);
 
  let path = config.value_of("INPUT").unwrap();
  let input = source?.load(path)?;
        
  // select source parser based on arguments
  let parser_type = config.value_of("PARSER").unwrap();
  let parser = parsers::Parser::new(parser_type, input);
  
  // run
  codacy::report(&parser.unwrap(), &config)
}
