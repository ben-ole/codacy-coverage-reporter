use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub struct Config {
    pub filename: String,
    pub path_prefix: String,
    pub output: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct FileReport {
    filename: String,
    total: f64,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

  let source = read_source( &config.filename )?;

  let cov = source["coverage"].clone();

  let target = select_target(&source)?;

  let body = json!({
    "total": cov,
    "fileReports": file_reports(&target, &config.path_prefix)
  });

  if let Some(ref path) = config.output {
    let mut file = File::create(path)?;
    file.write_all(body.to_string().as_bytes())?;  
  } else {
    println!("{}", body);
  }

  Ok(())
}

// Helper
impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 2 {
        return Err("usage: codacy-xcov path/source.json [path/output.json] [file_prefix]");
    }

    let filename = args[1].clone();


    let mut output = None;
    if args.len() > 2 {
      output = Some(args[2].clone());
    }

    let mut path_prefix = String::new();
    if args.len() > 3 {
      path_prefix = args[3].clone();
    }

    Ok(Config { filename, output, path_prefix })
  }
}

/// read xcov json report
fn read_source(path: &String) -> Result<serde_json::Value, Box<Error>> {

  let mut f = File::open(path)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  Ok( serde_json::from_str(&contents)? )
}

fn file_reports(source: &serde_json::Value, prefix: &String) -> serde_json::Value {  

  let files = &source["files"].as_array().unwrap();

  let files_cov: Vec<FileReport> = files.iter().map( |f| {
    let name = f["name"].as_str().unwrap();
    let filename = prefix.to_owned() + name;

    FileReport {  filename, 
                  total: f["coverage"].as_f64().unwrap() * 100.0 }

  }).collect();

  json!(files_cov)
}

fn select_target(source: &serde_json::Value) -> Result<&serde_json::Value, Box<Error>> {
  let targets = &source["targets"][0];
  Ok(targets)
}

