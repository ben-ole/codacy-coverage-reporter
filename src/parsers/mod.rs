use serde_json;

mod xcov;

pub enum Parser {
    Xcov(serde_json::Value)
}

impl Parser {

    // convenience initializer: load Parser from string with content
    pub fn new(from: &str, data: serde_json::Value) -> Result<Parser, &'static str> {

        match from {
            "XCOV" => Ok( Parser::Xcov(data["targets"][0].to_owned()) ),
                _  => Err ( "Not supported parser" )
        }
    }

    // get total coverage of project
    pub fn total_coverage(&self) -> u8 {

        match *self {
            Parser::Xcov(ref content) => xcov::total_cov(content)
        }
    }

    // get coverage per file
    pub fn file_coverage(&self, path_prefix: &str) -> serde_json::Value  {

        match *self {
            Parser::Xcov(ref content) => xcov::file_cov(&content, path_prefix)
        }
    }

}
