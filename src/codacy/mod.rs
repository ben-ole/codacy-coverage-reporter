use reqwest;
use hyper::header::{Headers, ContentType};
use std::error::Error;
use parsers;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const BASE_URI: &'static str = "https://api.codacy.com";

#[cfg(test)]
const BASE_URI: &'static str = mockito::SERVER_URL;

// codacy authentication header definition
header! { (ProjectToken, "project_token") => [String] }

// PUBLIC

// use the supplied parser to extract the coverage and send them over to
// to codacy.
pub fn report<'a>(  parser: &parsers::Parser, 
                    prefix: &str, 
                    output: Option<&str>, 
                    commit: &str, 
                    lang: &str, 
                    token: &str ) -> Result<(), Box<Error>> {

  let body = json!({
    "total": parser.total_coverage(),
    "fileReports": parser.file_coverage(prefix)
  });

  // write request payload file (optional) 
  if let Some(ref path) = output {
    let mut file = File::create(path)?;
    file.write_all(body.to_string().as_bytes())?;
  }

  send( body.to_string(), commit, lang, token )
}

// PRIVATE

// create request payload and send to codacy api
fn send(body: String, commit_uuid: &str, language: &str, project_token: &str)
            -> Result<(),Box<Error>> {

  let client = reqwest::Client::new();

  let path = BASE_URI.to_owned() + "/2.0/coverage/" + commit_uuid + "/" + language;
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

// UNIT TESTS

#[cfg(test)]
mod tests {

    use super::*;
    use mockito::mock;
    
    #[test]
    fn test_send() {
        let mock = mock_request();

        let send_result = send("test".to_string(), "123abc", "swift", "xyz");
        assert!(send_result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_report() {
        let test_parser = parsers::Parser::Test;
        let mock = mock_request();

        let report_result = report(&test_parser, "prefix", None, "123abc", "swift", "xyz");
        assert!(report_result.is_ok());
        mock.assert();
    }

    // utils
    fn mock_request() -> mockito::Mock {
        mock("POST", "/2.0/coverage/123abc/swift")
            .with_status(201)
            .with_header("content-type", "application/json")
            .create()
    }
}
