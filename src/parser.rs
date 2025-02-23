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
    /// The function returns the next `Token` reference in the input
    /// It is inteded to be used in a while-let loop
    /// 
    /// Returns None when the iteration is complete
    fn iterate(&mut self) -> Option<&Token> {
        if self.pos < self.input.len() {
            let token = &self.input[self.pos];
            self.pos += 1;
            return Some(token);
        } else {
            return None;
        }
    }

    /// Parsing
    /// 
    /// The function will return an nested `Environment` structure
    /// It iterates over all `Token`s in the input and matches them to the appropriate semantic structure
    pub fn parse_tokens(&mut self) -> Rc<Environment> {
        let global_env: Rc<Environment> = Environment::new(EnvironmentConfig::default());

        while let Some(token) = self.iterate() {
            match token {
                Token::Number(t) => {
                    match Parser::parse_integer(t) {
                        Ok(v) => global_env.add_element(EnvValue::INT(v)),
                        Err(e) => panic!("{e}"),
                    }
                },
                _ => {},
            }
        }

        return global_env;
    }

    /// Parse an integer
    /// 
    /// The function will return a signed system-sized integer or an error
    fn parse_integer(token: &String) -> Result<isize, ParserError> {
        return Ok(token.parse::<isize>()?);
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
}

/// Display custom errors
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::Int(e) => write!(f, "Error in parsing integer: {e}"),
            ParserError::Float(e) => write!(f, "Error in parsing float: {e}"),
        }
    }
}

/// Implement currying `ParserError`s with the `?` operator
impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::Int(e) => Some(e),
            ParserError::Float(e) => Some(e),
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

    #[test]
    fn matches_integer() {
        let lexed_input = Lexer::new(vec!["5".to_string()]).tokenize();
        let global_env = Parser::new(lexed_input).parse_tokens();
        assert_eq!(global_env.get_elements(), vec![EnvValue::INT(5)]);
    }
}