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
use parsers::Parser;

// PUBLIC

// parse source with given options and send coverage over to codacy
pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  // select source loader based on arguments
  let default_type = "JSON";
  let file_type = config.value_of("TYPE").unwrap_or(default_type);

  let source: Option<Box<source::Source>> = match file_type {
      "JSON" =>  Some(Box::new(source::json_file::JsonFile::new(&config))),
      "XML"  =>  Some(Box::new(source::xml_file::XmlFile::new(&config))),
      _      =>  None,
  };

  let input = source.unwrap().load()?;

  // select source parser based on arguments
  let default_parser = "XCOV";
  let parser_type = config.value_of("PARSER").unwrap_or(default_parser);

  let parser_option: Option<Box<parsers::Parser>> = match parser_type {
      "XCOV" =>  Some(Box::new(parsers::xcov::XCov::new(&input))),
      _      =>  None,
  };

  let parser = parser_option.unwrap();

  // run
  codacy::report(&parser, &config)
}
