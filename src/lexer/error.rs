use std::error::Error;
use std::fmt;

/// Error type for Envlang lexer
/// 
/// The error types match various failure states during lexical analysis
/// 
/// Errors always contain at least:
/// - The current lexer position
/// - The input length (if relevant)
/// 
/// Errors may optionally include information about:
/// - Expected and actual values
/// - Attempted operation parameters
#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String, usize),            // (token, pos)
    UnterminatedString(usize, String),      // (pos, partial_string)
    EmptyIdentifier(usize),                 // (pos)
    BrokenLexer(usize, usize),              // (pos, input_len)
    InvertedSlice(usize, usize),            // (start, end)
    SliceOutOfBounds(usize, usize, usize),  // (pos, end, input_len)
}

impl Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidToken(t, pos) => 
                write!(f, "Position {}: Invalid token: {}", pos, t),
            LexerError::UnterminatedString(pos, partial) => 
                write!(f, "Position {}: Unterminated string literal '{}'", pos, partial),
            LexerError::EmptyIdentifier(pos) => 
                write!(f, "Position {}: Empty identifier", pos),
            LexerError::BrokenLexer(pos, len) => 
                write!(f, "Lexer in invalid state: position {} beyond input length {}", pos, len),
            LexerError::InvertedSlice(start, end) => 
                write!(f, "Invalid slice: start position {} greater than end position {}", start, end),
            LexerError::SliceOutOfBounds(pos,end, len) => 
                write!(f, "Slice error: attempted to get position {} to {} from string with length {}", pos, end, len),
        }
    }
}