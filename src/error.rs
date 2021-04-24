use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SyntaxError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "[line {}] Error{}: {}",
            self.line, self.location, self.message
        )
    }
}

#[derive(Debug)]
pub struct GenerateError {
    pub line: usize,
    pub message: String,
}

impl Error for GenerateError {}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[line {}] Error: {}", self.line, self.message)
    }
}
