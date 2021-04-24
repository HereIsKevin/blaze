use std::fmt;

use crate::kind::Kind;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?} {}", self.kind, self.lexeme)
    }
}
