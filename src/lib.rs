extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate hyper;

use hyper::header::{Headers, ContentType};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct FileReport {
    filename: String,
    total: u8,
}

header! { (ProjectToken, "project_token") => [String] }

pub fn run(config: clap::ArgMatches) -> Result<(), Box<Error>> {

  let source = read_source( config.value_of("INPUT").unwrap() )?;

  let cov = source["coverage"].as_f64().unwrap() * 100.0;

  let target = select_target(&source)?;

  let body = json!({
    "total": cov.round() as u8,
    "fileReports": file_reports(&target, config.value_of("PREFIX").unwrap_or(""))
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

/// read xcov json report
fn read_source(path: &str) -> Result<serde_json::Value, Box<Error>> {

  let mut f = File::open(path)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  Ok( serde_json::from_str(&contents)? )
}

fn file_reports(source: &serde_json::Value, prefix: &str) -> serde_json::Value {

  let files = &source["files"].as_array().unwrap();

  let files_cov: Vec<FileReport> = files.iter().map( |f| {
    let name = f["name"].as_str().unwrap();
    let filename = prefix.to_owned() + name;

    let coverage = f["coverage"].as_f64().unwrap() * 100.0;
    FileReport { filename, total: coverage.round() as u8 }

  }).collect();

  json!(files_cov)
}

fn select_target(source: &serde_json::Value) -> Result<&serde_json::Value, Box<Error>> {
  let targets = &source["targets"][0];
  Ok(targets)
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
