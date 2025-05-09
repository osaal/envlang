//! The Envlang lexer
//! 
//! The lexer takes a Unicode-segmented `String` vector from [`segment_graphemes()`] and turns it into a vector of [`Token`]s.
//! 
//! These `Token`s are then intended to be [parsed] into an Abstract Syntax Tree.
//! 
//! # Error handling
//! 
//! The lexer provides [comprehensive error handling] for:
//! * Invalid tokens
//! * Unterminated strings
//! * Empty identifiers
//! * Unrecognized input
//! * Invalid lexer states
//! 
//! All errors include position information for reporting.
//! 
//! [comprehensive error handling]: ./enum.LexerError.html
//! [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
//! [`Token`]: ./enum.Token.html
//! [parsed]: ../parser/index.html

mod token;
mod error;
mod tests;

pub use token::Token;
pub use error::LexerError;

use crate::symbols::{Keywords, Booleans, ArithmeticOperators, ComparisonOperators, LogicalOperators, OtherOperators, Operators};
use std::rc::Rc;
use std::borrow::Borrow;

/// Envlang lexer
/// 
/// The `Lexer` struct holds the Unicode-segmented `String` vector from [`segment_graphemes()`].
/// 
/// Note, that the Lexer does not actually check whether the `String`s have been properly segmented.
/// 
/// # Panics
/// 
/// The method `tokenize` may panic if the `tokens` vector has to outgrow system-specific `isize::MAX` bytes.
/// 
/// # Errors
/// All lexer methods return `Result` types with detailed error information including:
/// * Position of the error in the input vector
/// * Type of error
/// * Relevant context (e.g., partial string content for unterminated strings)
/// 
/// [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
pub struct Lexer {
    input: Vec<Rc<str>>,
    current: usize
}

impl Lexer {
    /// Initializes a new Lexer with a given input vector
    /// 
    /// # Undefined Behaviour
    /// The Lexer will still work with a non-segmented input, but the results will not be accurate for many Unicode characters.
    pub fn new(input: Vec<String>) -> Self {
        Self {
            input: input.into_iter().map(|s| Rc::from(s.as_str())).collect(),
            current: 0
        }
    }

    /// Iterates over the input
    fn iterate(&mut self) -> Option<(usize, Rc<str>)> {
        if self.current < self.input.len() {
            let pos = self.current;
            let ch = Rc::clone(&self.input[self.current]);
            self.current += 1;
            return Some((pos, ch));
        } else {
            return None;
        }
    }

    /// Peeks at the Nth input
    /// 
    /// Used as an immutable and flexible alternative to [`iterate`](Lexer::iterate).
    /// 
    /// # Errors
    /// * [`LexerError::BrokenLexer`]: The currently-held position is beyond input length.
    /// * [`LexerError::IndexOutOfBounds`]: The index of the requested element is beyond input length.
    fn peek_n(&self, n: usize) -> Result<Rc<str>, LexerError> {
        // TODO: This should rather return an Option, since an empty symbol stream is a valid state.
        // However, calls outside of symbol stream length should still error...
        // How about Result<Option<Rc<str>>, LexerError>? That is quite verbose...
        let length: usize = self.input.len();
        if self.current > length {
            Err(LexerError::BrokenLexer(self.current, length))
        } else if n >= length {
            Err(LexerError::IndexOutOfBounds(self.current, n, length))
        } else {
            Ok(Rc::clone(&self.input[n]))
        }
    }

    /// Tokenizes the input
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`tokenize_string`](Lexer::tokenize_string), [`tokenize_operator`](Lexer::tokenize_operator), [`tokenize_number`](Lexer::tokenize_number), or [`tokenize_alphabetics`](Lexer::tokenize_alphabetics).
    /// * [`LexerError::UnrecognizedInput`]: The input string does not match the syntax of Envlang.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some((pos, unicode_string)) = self.iterate() {
            match unicode_string.borrow() {
                "{" =>
                    tokens.push(Token::LeftBrace),
                "}" =>
                    tokens.push(Token::RightBrace),
                "(" =>
                    tokens.push(Token::LeftParen),
                ")" =>
                    tokens.push(Token::RightParen),
                "[" =>
                    tokens.push(Token::LeftBracket),
                "]" =>
                    tokens.push(Token::RightBracket),
                "\"" =>
                    tokens.push(self.tokenize_string("\"", pos)?),
                "'" =>
                    tokens.push(self.tokenize_string("'", pos)?),
                "+" | "-" | "*" | "/" | "%" | "^" | "=" | "<" | ">" | "!" | "&" | "|" =>
                    tokens.push(self.tokenize_operator(&unicode_string, pos)?),
                "." =>
                    tokens.push(Token::Operator(Operators::Other(OtherOperators::ACCESSOR))),
                "," =>
                    tokens.push(Token::Comma),
                ";" =>
                    tokens.push(Token::LineTerminator),
                unicode_string if unicode_string.chars().all(|c| c.is_ascii_digit()) =>
                    tokens.push(self.tokenize_number(unicode_string, pos)?),
                unicode_string if unicode_string.chars().all(|c| c.is_alphabetic()) =>
                    tokens.push(self.tokenize_alphabetics(unicode_string, pos)?),
                unicode_string if unicode_string.chars().all(|c| c.is_whitespace()) =>
                    tokens.push(Token::Whitespace(Rc::from(unicode_string))),
                _ => return Err(LexerError::UnrecognizedInput(pos, unicode_string.to_string()))
            }
        }
        tokens.push(Token::EOF);
        return Ok(tokens);
    }

    /// Distinguishes single-symbol operators from dual-symbol operators.
    /// 
    /// # Errors
    /// * [`LexerError::InvalidOperator`]: Either the first or the second symbol did not match the set of valid comparison operator symbols.
    /// * [`LexerError::IndexOutOfBounds`]: There is no next symbol in the symbol queue, meaning there cannot be a right-hand-side to the operator.
    fn tokenize_comparison(&mut self, first_op: &str, pos: usize) -> Result<Token, LexerError> {
        if let Ok(next_symbol) = self.peek_n(self.current) {
            // We have a next symbol
            // Check whether it is "="
            match next_symbol.borrow() {
                "=" => {
                    // Increment `current` to avoid re-parsing the next symbol once the method is done.
                    self.current += 1;
                    match first_op {
                        ">" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::GEQ))),
                        "<" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::LEQ))),
                        "=" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::EQ))),
                        "!" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::NEQ))),
                        _ => return Err(LexerError::InvalidOperator(pos, first_op.to_string())),
                    }
                },
                _ => match first_op {
                    "!" => return Ok(Token::Operator(Operators::Logical(LogicalOperators::NOT))),
                    _ => return Err(LexerError::InvalidOperator(pos, next_symbol.to_string())),
                },
            }
        }

        // We should have a single symbol (or peek_n returned some other error...)
        match first_op {
            ">" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::GT))),
            "<" => return Ok(Token::Operator(Operators::Comparison(ComparisonOperators::LT))),
            "=" => return Ok(Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT))),
            "!" => return Ok(Token::Operator(Operators::Logical(LogicalOperators::NOT))),
            _ => return Err(LexerError::InvalidOperator(pos, first_op.to_string())),
        }
    }

    /// Matches one or more characters that conform to [`char::is_ascii_digit`]
    /// 
    /// # Errors
    /// * [`LexerError::InvalidToken`]: `unicode_string` is not an ASCII digit.
    fn tokenize_number(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        if unicode_string.is_empty() {
            return Err(LexerError::InvalidToken(pos, String::new()));
        }

        // Validate first character to exit early in case of inappropriate input
        if !unicode_string.chars().all(|c| c.is_ascii_digit()) {
            return Err(LexerError::InvalidToken(pos, unicode_string.to_string()));
        }

        let mut number = unicode_string.to_string();

        while let Ok(next_unicode_string) = self.peek_n(self.current) {
            if !next_unicode_string.chars().all(|c| c.is_ascii_digit()) {
                // Valid termination state, exit out.
                break;
            }

            let (_, digit) = self.iterate().unwrap();
            number.push_str(&digit);
        }

        return Ok(Token::Number(Rc::from(number)));
    }

    /// Matches potential non-string-delimited character sequences
    /// 
    /// Matches can be boolean values, reserved [keywords](crate::symbols::Keywords), or identifiers.
    /// 
    /// # Errors
    /// * [`LexerError::InvalidToken`]: The input stream did not start with an alphabetic character.
    /// * [`LexerError::EmptyIdentifier`]: The input stream is empty.
    fn tokenize_alphabetics(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        if unicode_string == "" {
            return Err(LexerError::EmptyIdentifier(pos));
        }

        if !unicode_string.chars().all(|c| c.is_alphabetic()) {
            return Err(LexerError::InvalidToken(pos, unicode_string.to_string()));
        }
        
        let mut temp = unicode_string.to_string();
        while let Ok(following_unicode_string) = self.peek_n(self.current) {
            match following_unicode_string.as_ref() {
                s if s.chars().all(
                    |c| c.is_alphanumeric()) ||
                    s == "-" ||
                    s == "_" =>
                    {
                        // TODO: Convert to if-let-some to handle error state more gracefully.
                        let (_, ch) = self.iterate().unwrap();
                        temp.push_str(&ch);
                    },
                    _ => break,
            }
        }

        match temp.as_str() {
            "let" => Ok(Token::Keyword(Keywords::LET)),
            "inherit" => Ok(Token::Keyword(Keywords::INHERIT)),
            "fun" => Ok(Token::Keyword(Keywords::FUN)),
            "true" => Ok(Token::Boolean(Booleans::TRUE)),
            "false" => Ok(Token::Boolean(Booleans::FALSE)),
            "return" => Ok(Token::Keyword(Keywords::RETURN)),
            _ => Ok(Token::Identifier(Rc::from(temp))),
        }
    }
    
    /// Tokenize a string
    /// 
    /// # Safety
    /// The calling context must supply an appropriate string delimiter.
    /// 
    /// # Arguments
    /// * `matched` - The delimiter used.
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`peek_n`](Lexer::peek_n).
    /// * [`LexerError::UnterminatedString`]: Input ends before a closing delimiter is found, or the lexer is broken.
    fn tokenize_string(&mut self, matched: &str, start_pos: usize) -> Result<Token, LexerError> {
        let mut value: String = String::new();
        loop {
            // TODO: Consider switching away from a generic loop.
            // TODO: Split up errors.
            match self.peek_n(self.current) {
                Ok(ch) => {
                    if ch.as_ref() != matched {
                        let (_, s) = self.iterate().unwrap();
                        value.push_str(&s);
                    } else {
                        self.iterate(); // Skip over the closing brace
                        break;
                    }
                },
                Err(LexerError::IndexOutOfBounds(_, _, _)) | Err(LexerError::BrokenLexer(_, _)) =>
                    return Err(LexerError::UnterminatedString(start_pos, value)),
                Err(e) => return Err(e),
            }
        }
        Ok(Token::StringLiteral(Rc::from(value)))
    }

    /// Tokenize an operator
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`tokenize_comparison`](Lexer::tokenize_comparison).
    /// * [`LexerError::UnrecognizedInput`]: The text did not match the set of valid operators.
    fn tokenize_operator(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        let operator = match unicode_string {
            "+" => Ok(Operators::Arithmetic(ArithmeticOperators::ADD)),
            "-" => Ok(Operators::Arithmetic(ArithmeticOperators::SUBTRACT)),
            "*" => Ok(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            "/" => Ok(Operators::Arithmetic(ArithmeticOperators::DIVIDE)),
            "%" => Ok(Operators::Arithmetic(ArithmeticOperators::MODULUS)),
            "^" => Ok(Operators::Arithmetic(ArithmeticOperators::EXPONENTIATION)),
            ">" | "<" | "!" | "=" => return Ok(self.tokenize_comparison(&unicode_string, pos)?),
            "&" => Ok(Operators::Logical(LogicalOperators::AND)),
            "|" => Ok(Operators::Logical(LogicalOperators::OR)),
            _ => Err(LexerError::UnrecognizedInput(pos, unicode_string.to_string())),
        };
        return Ok(Token::Operator(operator?));
    }
}