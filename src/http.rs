use reqwest;
use hyper::header::{Headers, ContentType};
use std::error::Error;

const BASE_URI: &'static str = "https://api.codacy.com/2.0/coverage/";
    
header! { (ProjectToken, "project_token") => [String] }

pub fn send(body: String, commit_uuid: &str, language: &str, project_token: &str)
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
