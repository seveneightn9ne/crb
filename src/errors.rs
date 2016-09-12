use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CrbError<'a> {
    description: &'a str,
}

impl<'a> fmt::Display for CrbError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Crb error: {}", self.description)
    }
}

impl<'a> CrbError<'a> {
    pub fn new(description: &str) -> CrbError {
        CrbError { description: description }
    }
}

impl<'a> Error for CrbError<'a> {
    fn description(&self) -> &str {
        self.description
    }

    // fn cause(self) -> Option<& Error> {
    //     None
    // }
}
