use serde_json;

mod xcov;

pub enum Parser {
    Xcov(serde_json::Value)
}

impl Parser {

    pub fn new(from: &str, data: serde_json::Value) -> Result<Parser, &'static str> {

        match from {
            "XCOV" => Ok( Parser::Xcov(data["targets"][0].to_owned()) ),
                _  => Err ( "Not supported parser" )
        }
    }

    pub fn total_coverage(&self) -> u8 {

        match *self {
            Parser::Xcov(ref content) => xcov::total_cov_xcov(content)
        }
    }

    pub fn file_coverage(&self, path_prefix: &str) -> serde_json::Value  {

        match *self {
            Parser::Xcov(ref content) => xcov::file_cov_xcov(&content, path_prefix)
        }
    }

}
