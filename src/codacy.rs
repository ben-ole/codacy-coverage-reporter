use reqwest;
use hyper::header::{Headers, ContentType};
use std::error::Error;
use parsers;
use clap;
use std::fs::File;
use std::io::prelude::*;

const BASE_URI: &'static str = "https://api.codacy.com/2.0/coverage/";
    
header! { (ProjectToken, "project_token") => [String] }

pub fn report<'a, T: parsers::Parser<'a>>(parser: &T, config: clap::ArgMatches)
         -> Result<(), Box<Error>> {

  let path_prefix = config.value_of("PREFIX").unwrap_or("");

  let body = json!({
    "total": parser.total_coverage(),
    "fileReports": parser.file_coverage(path_prefix)
  });

  // write request payload file (optional) 
  if let Some(ref path) = config.value_of("OUTPUT") {
    let mut file = File::create(path)?;
    file.write_all(body.to_string().as_bytes())?;
  }

  send( body.to_string(),
        config.value_of("COMMIT").unwrap(),
        config.value_of("LANGUAGE").unwrap_or("swift"),
        config.value_of("PROJECT_TOKEN").unwrap() )
    
}

fn send(body: String, commit_uuid: &str, language: &str, project_token: &str)
            -> Result<(),Box<Error>> {

  let client = reqwest::Client::new();

  let path = BASE_URI.to_owned() + commit_uuid + "/" + language;
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
