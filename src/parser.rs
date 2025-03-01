//! Envlang parser
//! 
//! The parser takes a tokenized input and turns it into semantically meaningful [`Environment`]s.
//! 
//! The [`Parser`] struct implements most of the work, with both methods and associated functions.
//! 
//! The error type [`ParserError`] is used to match against failure to parse specific Envlang symbols into Rust data types
//! 
//! [`Parser`]: ./struct.Parser.html
//! [`Environment`]: ../environment/struct.Environment.html
//! [`ParserError`]: ./enum.ParserError.html

use std::rc::Rc;
use std::fmt;
use std::error::Error;

use crate::lexer::Token;
use crate::environment::Environment;
use crate::environment::EnvironmentConfig;
use crate::environment::EnvName;
use crate::environment::EnvScope;
use crate::environment::EnvValue;
use crate::symbols::Booleans;

/// Envlang parser implementation
/// 
/// The `Parser` struct holds the tokenized input from `envlang::lexer::Lexer.tokenize()`
/// It also keeps track of the current position in the input vector
/// 
/// # Panics
/// 
/// When parsing tokens, if the parser fails to parse to the appropriate Rust data type, it will curry a `ParserError` to the `parse_tokens` method. This method will then panic with the `ParserError` information.
/// 
/// # Errors
/// 
/// All parsing sub-functions return `Result<T, ParserError>`. These errors are handled in the `parse_tokens` method, usually by panic.
pub struct Parser {
    input: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Parser initialization
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            pos: 0
        }
    }

    /// Iteration
    /// 
    /// The function returns the `Token` at the current `pos` index
    /// It is intended to be used in a while-let loop
    /// 
    /// Returns None when the iteration is complete
    fn get_token(&self) -> Option<&Token> {
        if self.pos < self.input.len() {
            let token = &self.input[self.pos];
            return Some(token);
        } else {
            return None;
        }
    }

    /// Increment position
    /// 
    /// The function increments the position
    /// 
    /// It is intended to be used after successful readings of the input vector
    fn increment_pos(&mut self) {
        self.pos += 1;
    }

    /// Parsing
    /// 
    /// The function will return an nested `Environment` structure
    /// It iterates over all `Token`s in the input and matches them to the appropriate semantic structure
    /// 
    /// NB: Position incrementation has to happen in the inner-most parsing functions!
    pub fn parse_tokens(&mut self) -> Rc<Environment> {
        let global_env: Rc<Environment> = Environment::new(EnvironmentConfig::default());

        // Cases to still cover:
        // Token::Identifier(v) -> interpret identifier name, make sure it's unique, create new environment
        // Token::LeftBrace/Token::RightBrace -> encapsulate new environment
        // Token::Keyword(v) -> interpret keyword, check its requirements, create new environment if necessary
        // Token::Operator(v) -> interpret operator
        // Token::Whitespace(v) -> interpret whitespace, skip?
        // Token::FullStop -> interpret fullstop as accessor operator
        // Token::EOF -> do nothing, since we're done

        while let Some(token) = self.get_token() {
            match token {
                Token::Number(_) => {
                    match self.parse_integer() {
                        Ok(v) => global_env.add_element(v),
                        Err(e) => panic!("{e}"),
                    }
                },
                Token::StringLiteral(_) => {
                    match self.parse_string() {
                        Ok(v) => global_env.add_element(v),
                        Err(e) => panic!("{e}"),
                    }
                },
                Token::Boolean(_) => {
                    match self.parse_boolean() {
                        Ok(v) => global_env.add_element(v),
                        Err(e) => panic!("{e}"),
                    }
                }
                _ => {self.increment_pos()},
            }
        }

        return global_env;
    }

    /// Parse an integer
    /// 
    /// The function will return a signed system-sized integer or an error
    /// 
    /// NOTE: Because it converts floats by recognising a full-stop after a number, take care to run this AFTER matching identifiers!
    fn parse_integer(&mut self) -> Result<EnvValue, ParserError> {
        let mut array: String = String::new();
        let wrapped: EnvValue;
        let mut is_float: bool = false;

        while let Some(token) = self.get_token() {
            match token {
                Token::Number(t) => array.push_str(t),
                Token::FullStop => {
                    is_float = true;
                    array.push_str(".");
                },
                _ => break,
            }
            self.increment_pos();
        }

        if is_float {
            let result = array.parse::<f64>()?;
            wrapped = EnvValue::FLOAT(result);
        } else {
            let result = array.parse::<isize>()?;
            wrapped = EnvValue::INT(result);
        }

        return Ok(wrapped);
    }

    /// Parse a string literal
    /// 
    /// The function will return a String value or an error
    fn parse_string(&mut self) -> Result<EnvValue, ParserError> {
        let result: Result<EnvValue, ParserError>;
        if let Some(token) = self.get_token() {
            match token {
                Token::StringLiteral(t) => {
                    result = Ok(EnvValue::STRING(t.to_string()));
                    self.increment_pos();
                },
                _ => result = Err(ParserError::TokenTypeMismatch), // This branch should never occur!
            }
        } else {
            result = Err(ParserError::NoToken);
        }
        return result;
    }

    /// Parse a boolean
    /// 
    /// The function will return a boolean value or an error
    fn parse_boolean(&mut self) -> Result<EnvValue, ParserError> {
        let result: Result<EnvValue, ParserError>;

        if let Some(token) = self.get_token() {
            match token {
                Token::Boolean(bool) => {
                    // Match which boolean
                    match bool {
                        Booleans::TRUE => {
                            result = Ok(EnvValue::BOOL(true));
                            self.increment_pos();
                        },
                        Booleans::FALSE => {
                            result = Ok(EnvValue::BOOL(false));
                            self.increment_pos();
                        },
                        _ => result = Err(ParserError::InvalidBoolean),
                    }
                },
                _ => result = Err(ParserError::TokenTypeMismatch),
            }
        } else {
            result = Err(ParserError::NoToken);
        }
        return result;
    }
}

/// Error type for Envlang parser
/// 
/// The error types match the attempted parsing from Envlang into Rust atomics
/// 
/// In general, the error types wrap `std` library errors
#[derive(Debug)]
pub enum ParserError {
    Int(std::num::ParseIntError),
    Float(std::num::ParseFloatError),
    NoToken,
    TokenTypeMismatch,
    InvalidBoolean,
}

/// Display custom errors
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::Int(e) => write!(f, "Error in parsing integer: {e}"),
            ParserError::Float(e) => write!(f, "Error in parsing float: {e}"),
            ParserError::NoToken => write!(f, "No token available in queue"),
            ParserError::TokenTypeMismatch => write!(f, "Token type mismatch"),
            ParserError::InvalidBoolean => write!(f, "Invalid boolean lexing"),
        }
    }
}

/// Implement currying `ParserError`s with the `?` operator
impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::Int(e) => Some(e),
            ParserError::Float(e) => Some(e),
            ParserError::NoToken => None,
            ParserError::TokenTypeMismatch => None,
            ParserError::InvalidBoolean => None,
        }
    }
}

/// Convert `ParseIntError` into `ParserError`
impl From<std::num::ParseIntError> for ParserError {
    fn from(e: std::num::ParseIntError) -> ParserError {
        ParserError::Int(e)
    }
}

/// Convert `ParseFloatError` into `ParserError`
impl From<std::num::ParseFloatError> for ParserError {
    fn from(e: std::num::ParseFloatError) -> ParserError {
        ParserError::Float(e)
    }
}

// Unit tests for parser.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    use crate::environment::*;

    use std::time::Instant;

    #[test]
    fn matches_integer() {
        let lexed_input = Lexer::new(vec!["5".to_string()]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::INT(5)]);
    }

    #[test]
    fn matches_float() {
        let lexed_input = Lexer::new(vec!["3".to_string(), ".".to_string(), "5".to_string()]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::FLOAT(3.5)]);
    }

    #[test]
    fn matches_multidigit_float() {
        let lexed_input = Lexer::new(vec![
            "123".to_string(), 
            ".".to_string(), 
            "456".to_string()
        ]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::FLOAT(123.456)]);
    }

    #[test]
    fn matches_nontrailingzero_float() {
        let lexed_input = Lexer::new(vec![
            "123".to_string(), 
            ".".to_string()
        ]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::FLOAT(123.0)]);
    }

    #[test]
    fn matches_string() {
        let lexed_input = Lexer::new(vec![
            "\"".to_string(),
            "a".to_string(),
            "s".to_string(),
            "d".to_string(),
            "\"".to_string()
        ]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::STRING("asd".to_string())])
    }

    #[test]
    fn matches_bool_true() {
        let lexed_input = Lexer::new(vec![
            "t".to_string(),
            "r".to_string(),
            "u".to_string(),
            "e".to_string()
        ]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::BOOL(true)]);
    }

    #[test]
    fn matches_bool_false() {
        let lexed_input = Lexer::new(vec![
            "f".to_string(),
            "a".to_string(),
            "l".to_string(),
            "s".to_string(),
            "e".to_string()
        ]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::BOOL(false)]);
    }

    // This test was to benchmark two alternative approaches in handling lexing-parsing responsibilities
    // Approach 1: Lexer also covers combining numbers together
    // Approach 2: Lexer only tokenizes single Unicode characters
    // Results suggest that approach 1 is almost double the speed
    // Leaving the test here for future reference
    // TODO: Move this to an integration test once I've moved from pure-binary to binary-library-combined crate structure.
    #[test]
    fn benchmark_number_parsing() {
        // Create a large input with many numbers
        let large_input = vec!["96145225658".to_string()];
        
        // Approach 1: Combined tokens
        let start = Instant::now();
        let tokens = Lexer::new(large_input.clone()).tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_tokens();
        let combined_duration = start.elapsed();

        // Approach 2: Single character tokens
        let start = Instant::now();
        let char_input: Vec<String> = large_input
            .iter()
            .flat_map(|s| s.chars().map(|c| c.to_string()))
            .collect();
        let tokens = Lexer::new(char_input).tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_tokens();
        let single_char_duration = start.elapsed();

        println!("Combined tokens: {:?}", combined_duration);
        println!("Single char tokens: {:?}", single_char_duration);
    }
}