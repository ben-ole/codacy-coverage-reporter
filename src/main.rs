extern crate clap;
extern crate codacy_xcov;

use std::process;

use clap::{Arg, App};

fn main(){

  let matches = App::new("Codacy xcov supply")
        .version("1.0")
        .author("Benjamin M. <benjamin@7mind.de>")
        .about("Parse swift coverage (xcov) and send over to codacy.")
        .arg(Arg::with_name("PREFIX")
             .long("prefix")
             .value_name("PREFIX")
             .help("Prefix the path to the files in the output json with given slice.")
             .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
             .short("o")
             .long("output")
             .value_name("OUTPUT")
             .help("save compiled json to `output`.")
             .takes_value(true))
        .arg(Arg::with_name("language")
             .short("l")
             .long("language")
             .value_name("LANGUAGE")
             .help("change language parameter for codacy. defaults to `swift`")
             .takes_value(true))
        .arg(Arg::with_name("input_type")
             .short("t")
             .long("type")
             .value_name("TYPE")
             .help("Set input file type. Available: `json`. defaults to `json`")
             .takes_value(true))
        .arg(Arg::with_name("parser")
             .short("p")
             .long("parser")
             .value_name("PARSER")
             .help("Set parser to use for coverage extraction. Available: `xcov`. defaults to `xcov`")
             .takes_value(true))
        .arg(Arg::with_name("PROJECT_TOKEN")
             .help("Set the codacy project token")
             .required(true)
             .index(1))
        .arg(Arg::with_name("COMMIT")
             .help("Set the current GIT Commit UUID")
             .required(true)
             .index(2))
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .required(true)
             .index(3))        
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();

  if let Err(e) = codacy_xcov::run(matches) {
    println!("Application error: {}", e);
    process::exit(1);
  }

}

