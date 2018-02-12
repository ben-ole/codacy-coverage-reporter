use serde_json;

mod xcov;

#[cfg(test)]
mod test_double;

pub enum Parser {
    Xcov(serde_json::Value),

    #[cfg(test)]
    Test
}

impl Parser {

    // convenience initializer: load Parser from string with content
    pub fn new(from: &str, data: serde_json::Value) -> Result<Parser, &'static str> {

        match from {
            "XCOV" => Ok( Parser::Xcov(data["targets"][0].to_owned()) ),
                _  => Err( "Not supported parser" )
        }
    }

    // get total coverage of project
    pub fn total_coverage(&self) -> u8 {

        match *self {
            Parser::Xcov(ref content) => xcov::total_cov(content),

            #[cfg(test)]
            Parser::Test => test_double::total_cov()
        }
    }

    // get coverage per file
    pub fn file_coverage(&self, path_prefix: &str) -> serde_json::Value  {

        match *self {
            Parser::Xcov(ref content) => xcov::file_cov(&content, path_prefix),

            #[cfg(test)]
            Parser::Test => test_double::file_cov()
        }
    }

}
