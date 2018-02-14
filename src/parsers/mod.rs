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

// UNIT TESTS

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_xcov() {
        let sample_data = json!({"targets":[{"test": 42}]});

        let actual_result = Parser::new("XCOV", sample_data);

        assert!(actual_result.is_ok());        
        assert!(matches!(actual_result.unwrap(), Parser::Xcov(_)));        
    }

    #[test]
    fn test_new_bad() {
        let sample_data = json!({"targets":[{"test": 42}]});

        let actual_result = Parser::new("BAD", sample_data);

        assert!(actual_result.is_err());
    }

    #[test]
    fn test_total_cov_xcov() {
        let sample_data = json!({"targets":[{"coverage": 0.42}]});

        let xcov_parser = Parser::new("XCOV", sample_data).unwrap();
        assert_eq!(xcov_parser.total_coverage(), 42);
    }

    #[test]
    fn test_file_coverage_xcov() {
        let sample_data = json!({"files": []});

        let xcov_parser = Parser::Xcov(sample_data);
        assert!(matches!(xcov_parser.file_coverage("/path"), serde_json::Value::Array(_)));
    }
}
