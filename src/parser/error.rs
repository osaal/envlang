use std::error::Error;
use std::fmt;

/// Error type for Envlang parser
/// 
/// The error types match various failure states during semantic analysis
/// 
/// Errors always contain at least:
/// - The current parser position
/// - The current line number (calculated by the parser)
/// - The value that caused the error
/// 
/// Errors may optionally include information about:
/// - Expected and actual values
/// - Attempted operation parameters
#[derive(Debug, PartialEq)]
pub enum ParserError {
    NotANumber(usize, usize, String),           // (pos, line, value)
    MalformedNumber(usize, usize, String),      // (pos, line, value)
    InvalidOperation(usize, usize, String),     // (pos, line, value)
    BinaryOpWithNoLHS(usize, usize),            // (pos, line)
    BinaryOpWithNoRHS(usize, usize),            // (pos, line)
    WhitespaceInNumber(usize, usize, String),   // (pos, line, value)
    ParserLogicError(usize, usize),             // (pos, line)
}

impl Error for ParserError {}

/// TODOs:
/// - MalformedNumber is not informative enough, the context is not visible -> Need to reconstruct the line (through tokens or source)
/// - NotANumber is not informative enough, the context is not visible -> Need to reconstruct the line (through tokens or source)
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::NotANumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Value '{}' is not a number", line, pos, valuestr),
            ParserError::MalformedNumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Value '{}' is a malformed number", line, pos, valuestr),
            ParserError::InvalidOperation(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Invalid operation '{}'", line, pos, valuestr),
            ParserError::BinaryOpWithNoLHS(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Binary operation with no left-hand side", line, pos),
            ParserError::BinaryOpWithNoRHS(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Binary operation with no right-hand side", line, pos),
            ParserError::WhitespaceInNumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Whitespace inside number '{}'", line, pos, valuestr),
            ParserError::ParserLogicError(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Internal parser logic error", line, pos),
        }
    }
}