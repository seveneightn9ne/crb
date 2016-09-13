use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CrbError {
    description: String,
}

pub type CrbResult<T> = Result<T, CrbError>;

impl fmt::Display for CrbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Crb error: {}", self.description)
    }
}

impl CrbError {
    pub fn new(description: &str) -> CrbError {
        CrbError { description: description.to_string() }
    }
}

impl Error for CrbError {
    fn description(&self) -> &str {
        &self.description
    }
}
