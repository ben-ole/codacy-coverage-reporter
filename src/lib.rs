extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate hyper;

mod parsers;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

use hyper::header::{Headers, ContentType};

use parsers::Parser;

header! { (ProjectToken, "project_token") => [String] }

pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  let source = read_source( config.value_of("INPUT").unwrap() )?;

  let path_prefix = config.value_of("PREFIX").unwrap_or("");
  let mut parser: parsers::xcov::XCov = parsers::Parser::new(&source);

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

  send( body.to_string(),
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

fn send(body: String, commit_uuid: &str, language: &str, project_token: &str) -> Result<(),Box<Error>> {
  let client = reqwest::Client::new();

  let path = "https://api.codacy.com/2.0/coverage/".to_owned() + commit_uuid + "/" + language;
  let url = reqwest::Url::parse(&path)?;

  let bytes = body.into_bytes();

  let mut headers = Headers::new();
  headers.set(ContentType::json());
  headers.set(ProjectToken(project_token.to_string()));

  let req = client.post(url)
    .body(bytes)
    .headers(headers)
    .build()?;

  println!("request = {:?}", req);

  let mut resp = client.execute(req)?;

  println!("response = {:?}", resp.text()?);

  Ok(())
}
