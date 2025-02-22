use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParserError {
    Int(std::num::ParseIntError),
    Float(std::num::ParseFloatError),
}

// DISPLAY FUNCTIONS
// Implements displaying of custom errors
// Enumerate each error in the match statement and define the displayed message
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::Int(e) => write!(f, "Parsing error: {e}"),
            ParserError::Float(e) => write!(f, "Parsing error: {e}"),
        }
    }
}

// Implements currying errors with the `?` operator
impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::Int(e) => Some(e),
            ParserError::Float(e) => Some(e),
        }
    }
}

// CONVERSION FUNCTIONS
// Make one function per inherited error in ParserError

// Converts ParseIntErrors into ParserError
impl From<std::num::ParseIntError> for ParserError {
    fn from(e: std::num::ParseIntError) -> ParserError {
        ParserError::Int(e)
    }
}

// Converts ParseFloatErrors into ParserError
impl From<std::num::ParseFloatError> for ParserError {
    fn from(e: std::num::ParseFloatError) -> ParserError {
        ParserError::Float(e)
    }
}

// Unit tests for parsererror.rs
#[cfg(test)]
mod tests {
    use super::*;
}