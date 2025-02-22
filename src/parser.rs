use crate::errortypes::ParserError;

/// Types of parsed input
/// 
/// -  INT: Integer of type `i32`
/// -  FLOAT: Floating-point value of type `f64`
/// -  STRING: String of type `String`
/// -  BOOL: Boolean of type `bool`
#[derive(Debug, Clone)]
pub enum ParsedInputType {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
}

// Parsers that check if a given input String/string slice conforms to valid syntax

// Parsers that read given input String/string slices and return parsed inputs

/// Parse Rust string into Envlang integer
/// 
/// - input: String slice or String
/// 
/// Return: `Result<ParsedInputType::INT(input), ParserError::Int(error)>`
pub fn parse_int(input: &str) -> Result<ParsedInputType, ParserError> {
    return Ok(ParsedInputType::INT(input.parse::<i32>()?));
}

/// Parse Rust string into Envlang float
/// 
/// - input: String slice or String
/// 
/// Return: `Result<ParsedInputType::FLOAT(input), ParserError::Float(error)>`
pub fn parse_float(input: &str) -> Result<ParsedInputType, ParserError> {
    return Ok(ParsedInputType::FLOAT(input.parse::<f64>()?));
}

// Unit tests for parser.rs
#[cfg(test)]
mod tests {
    use super::*;
}