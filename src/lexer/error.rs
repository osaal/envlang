use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
    UnterminatedString,
    EmptyIdentifier,
}

impl Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidToken(t) => write!(f, "Invalid token: {}", t),
            LexerError::UnterminatedString => write!(f, "Unterminated string literal"),
            LexerError::EmptyIdentifier => write!(f, "Empty identifier"),
        }
    }
}