use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    error_message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.error_message)
    }
}

impl ParseError {
    pub fn new(error_message: String) -> Self {
        Self { error_message }
    }
}

impl Error for ParseError {
}
