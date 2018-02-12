extern crate clap;
extern crate reqwest;

#[cfg(test)]
extern crate mockito;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate hyper;

use std::error::Error;

mod parsers;
mod codacy;
mod source;

// parse source with given options and send coverage over to codacy
pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  // select source loader based on arguments
  let file_type = config.value_of("TYPE").unwrap_or("JSON");
  let source = source::SourceType::new(file_type);
 
  let path = config.value_of("INPUT").unwrap();
  let input = source?.load(path)?;
        
  // select source parser based on arguments
  let parser_type = config.value_of("PARSER").unwrap_or("XCOV");
  let parser = parsers::Parser::new(parser_type, input);
  
  // run
  codacy::report( &parser.unwrap(), 
                  config.value_of("PREFIX").unwrap_or(""),
                  config.value_of("OUTPUT"),
                  config.value_of("COMMIT").unwrap(),
                  config.value_of("LANGUAGE").unwrap_or("swift"),
                  config.value_of("PROJECT_TOKEN").unwrap() )
}
