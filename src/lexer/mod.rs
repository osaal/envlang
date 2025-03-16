//! The Envlang lexer
//! 
//! The lexer takes a Unicode-segmented `String` vector from [`segment_graphemes()`] and turns it into a vector of [`Token`]s.
//! 
//! These `Token`s are then intended to be [parsed] into an Abstract Syntax Tree.
//! 
//! # Error handling
//! 
//! The lexer provides [comprehensive error handling] for:
//! - Invalid tokens
//! - Unterminated strings
//! - Empty identifiers
//! - Unrecognized input
//! - Invalid lexer states
//! 
//! All errors include position information for reporting.
//! 
//! [comprehensive error handling]: ./enum.LexerError.html
//! [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
//! [`Token`]: ./enum.Token.html
//! [parsed]: ../parser/index.html

mod token;
mod error;

pub use token::Token;
pub use error::LexerError;

use crate::symbols::{Keywords, Booleans, ArithmeticOperators, OtherOperators, Operators, ReservedSymbols};
use std::rc::Rc;
use std::borrow::Borrow;

/// Envlang lexer
/// 
/// The `Lexer` struct holds the Unicode-segmented `String` vector from [`segment_graphemes()`].
/// 
/// Note, that the Lexer does not actually check whether the `String`s have been properly segmented!
/// 
/// # Panics
/// 
/// The method `tokenize` may panic if the `tokens` vector has to outgrow system-specific `isize::MAX` bytes.
/// 
/// # Errors
/// All lexer methods return `Result` types with detailed error information including:
/// - Position of the error in the input vector
/// - Type of error
/// - Relevant context (e.g., partial string content for unterminated strings)
/// 
/// [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
pub struct Lexer {
    input: Vec<Rc<str>>,
    current: usize
}

impl Lexer {
    /// Lexer initialization
    /// 
    /// Initializes a new Lexer with a given input vector
    /// 
    /// # Arguments
    /// * `input` - A vector of Strings to be conformed to `Rc<str>`s and lexed
    /// 
    /// # Returns
    /// A `Lexer` ready for lexing iteration
    /// 
    /// # Undefined Behaviour
    /// The Lexer will still work with a non-segmented input, but the results will not be accurate for many Unicode characters
    pub fn new(input: Vec<String>) -> Self {
        Self {
            input: input.into_iter().map(|s| Rc::from(s.as_str())).collect(),
            current: 0
        }
    }

    /// Iterate over the input
    /// 
    /// Returns both the position and content of the current token in the input vector.
    /// 
    /// # Returns
    /// A tuple of (position, content), where:
    /// - position: index of the current token
    /// - content: the token's content as an `Rc<str>`
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

    /// Peek at the Nth input
    /// 
    /// Used as an immutable and flexible alternative to `iterate`
    /// 
    /// # Arguments
    /// * `n` - the n-th input to retrieve
    /// 
    /// # Returns
    /// A Result containing a successfully retrieved element from the input vector
    /// 
    /// # Errors
    /// - `LexerError::BrokenLexer` if the currently-held position is beyond input length
    /// - `LexerError::IndexOutOfBounds` if the index of the requested element is beyond input length
    fn peek_n(&self, n: usize) -> Result<Rc<str>, LexerError> {
        let length: usize = self.input.len();
        if self.current > length {
            Err(LexerError::BrokenLexer(self.current, length))
        } else if n >= length {
            Err(LexerError::IndexOutOfBounds(self.current, n, length))
        } else {
            Ok(Rc::clone(&self.input[n]))
        }
    }

    /// Tokenize the input
    /// 
    /// Walks over the `input` vector and tokenizes it according to the syntax of Envlang.
    /// 
    /// The function relies on a number of `tokenize_` methods to accurately classify the input.
    /// These methods all return Result<Token, LexerError>, where the errors are curried to the return of this method.
    /// See the documentation of each `tokenize_` method for their potential error values.
    /// 
    /// # Returns
    /// A Result containing a vector of appropriately matched Tokens
    /// 
    /// # Errors
    /// - `LexerError::UnrecognizedInput` if a given input string does not match the syntax of Envlang
    /// - Errors curried from the `tokenize_` methods
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some((pos, unicode_string)) = self.iterate() {
            match unicode_string.borrow() {
                "{" =>
                    tokens.push(Token::LeftBrace(ReservedSymbols::ENVOPEN)),
                "}" =>
                    tokens.push(Token::RightBrace(ReservedSymbols::ENVCLOSE)),
                "(" =>
                    tokens.push(Token::LeftParen(ReservedSymbols::INHERITOPEN)),
                ")" =>
                    tokens.push(Token::RightParen(ReservedSymbols::INHERITCLOSE)),
                "\"" =>
                    tokens.push(self.tokenize_string("\"", pos)?),
                "'" =>
                    tokens.push(self.tokenize_string("'", pos)?),
                "+" | "-" | "*" | "/" | "%" | "^" | "=" =>
                    tokens.push(self.tokenize_operator(&unicode_string, pos)?),
                "." =>
                    tokens.push(Token::FullStop(OtherOperators::ACCESSOR)),
                "," =>
                    tokens.push(Token::Comma),
                ";" =>
                    tokens.push(Token::LineTerminator(ReservedSymbols::TERMINATOR)),
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

    /// Tokenize a sequence of numbers
    /// 
    /// Matches one or more characters that conform to `char.is_ascii_digit()`
    /// 
    /// # Arguments
    /// * `unicode_string` - The `str` to tokenize
    /// 
    /// # Returns
    /// A Result containing a successfully parsed `Token::Number`
    /// 
    /// # Errors
    /// - `LexerError::InvalidToken` if the given `unicode_string` is not an ASCII digit
    /// 
    /// The function is guaranteed to gracefully and silently handle other potential scenarios, so no further error conditions are necessary
    fn tokenize_number(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        // Exit early on empty string
        if unicode_string.is_empty() { return Err(LexerError::InvalidToken(pos, String::new())); }

        // Validate first character to exit early in case of inappropriate input
        // TODO: Write test to check this error's pos value, in case we are off-by-one
        if !unicode_string.chars().all(|c| c.is_ascii_digit()) { return Err(LexerError::InvalidToken(pos, unicode_string.to_string())); }

        let mut number = unicode_string.to_string();

        // We exit out in three conditions:
        // 1. Hitting a non-digit character (valid termination, no error)
        // 2. Reaching end of input (valid termination, no error)
        // 3. Invalid lexer state (caught by the while-let loop, no error)
        while let Ok(next_unicode_string) = self.peek_n(self.current) {
            if !next_unicode_string.chars().all(|c| c.is_ascii_digit()) {
                break;
            }
            let (_, digit) = self.iterate().unwrap();
            number.push_str(&digit);
        }
        return Ok(Token::Number(Rc::from(number)));
    }

    /// Tokenize a sequence of alphanumeric characters
    /// 
    /// Matches potential non-string-delimited character sequences:
    /// 1. Boolean values (true, false)
    /// 2. Reserved keywords (see symbols::Keywords)
    /// 3. Identifiers (a sequence of alphanumeric characters, may contain symbols::GenericSymbol)
    /// 
    /// # Arguments
    /// * `unicode_string` - The `str` to tokenize
    /// 
    /// # Returns
    /// A Result containing a successfully parsed Token of types:
    /// - `Token::Keyword` if a keyword was recognised
    /// - `Token::Boolean` if a boolean value was recognised
    /// - `Token::Identifier` in other cases (if no errors were caused)
    /// 
    /// # Errors
    /// - `LexerError::InvalidToken` if the input stream did not start with an alphabetic character
    /// - `LexerError::EmptyIdentifier` if the input stream is empty
    fn tokenize_alphabetics(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        if unicode_string == "" { return Err(LexerError::EmptyIdentifier(pos)); }

        // Doesn't start with an alphabetic character == invalid identifier/boolean/keyword
        if !unicode_string.chars().all(|c| c.is_alphabetic()) { return Err(LexerError::InvalidToken(pos, unicode_string.to_string())); }
        
        // The set of valid symbols is that of an identifier, with booleans and keywords having subsets of identifiers
        let mut temp = unicode_string.to_string();
        while let Ok(following_unicode_string) = self.peek_n(self.current) {
            match following_unicode_string.as_ref() {
                s if s.chars().all(
                    |c| c.is_alphanumeric()) ||
                    s == "-" ||
                    s == "_" =>
                    {
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
            _ => Ok(Token::Identifier(Rc::from(temp))),
        }
    }
    
    /// Tokenize a string
    /// 
    /// Matches one or more characters in-between opening and closing string literal delimiters.
    /// 
    /// Note, that the function actually never specifies which delimiter to use! It is up to the calling context to supply an appropriate delimiter.
    /// 
    /// # Arguments
    /// * `matched` - The delimiter used.
    /// 
    /// # Returns
    /// A Result containing a successfully parsed `Token::StringLiteral`
    /// 
    /// # Errors
    /// - `LexerError::UnterminatedString` if the input ends before a closing delimiter is found, or if the lexer is broken
    /// - Other errors curried from `peek_n()`
    fn tokenize_string(&mut self, matched: &str, start_pos: usize) -> Result<Token, LexerError> {
        let mut value: String = String::new();
        loop {
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

    fn tokenize_operator(&mut self, unicode_string: &str, pos: usize) -> Result<Token, LexerError> {
        let operator = match unicode_string {
            "+" => Ok(Operators::Arithmetic(ArithmeticOperators::ADD)),
            "-" => Ok(Operators::Arithmetic(ArithmeticOperators::SUBTRACT)),
            "*" => Ok(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            "/" => Ok(Operators::Arithmetic(ArithmeticOperators::DIVIDE)),
            "%" => Ok(Operators::Arithmetic(ArithmeticOperators::MODULUS)),
            "^" => Ok(Operators::Arithmetic(ArithmeticOperators::EXPONENTIATION)),
            "=" => Ok(Operators::Other(OtherOperators::ASSIGNMENT)),
            _ => Err(LexerError::UnrecognizedInput(pos, unicode_string.to_string()))?,
        };
        return Ok(Token::Operator(operator?));
    }
}

// Unit tests for lexer.rs
#[cfg(test)]
mod tests {
    use crate::symbols::ArithmeticOperators;
    use super::*;

    // Error condition tests
    #[test]
    fn error_empty_string() {
        let input = vec!["".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens,
            Err(LexerError::InvalidToken(pos, s)) if pos == 0 && s.is_empty()
        ));
    }

    #[test]
    fn error_unterminated_string() {
        let input = vec!["\"".to_string(), "hello".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens, 
            Err(LexerError::UnterminatedString(pos, s)) if pos == 0 && s == "hello"
        ));
    }

    #[test]
    fn error_special_character() {
        let input = vec!["@".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens, 
            Err(LexerError::UnrecognizedInput(pos, s)) if pos == 0 && s == "@"
        ));
    }

    // Tests for correct behaviour
    #[test]
    fn matches_left_brace() {
        let input = vec!["{".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LeftBrace(ReservedSymbols::ENVOPEN), Token::EOF]);
    }

    #[test]
    fn matches_right_brace() {
        let input = vec!["}".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::RightBrace(ReservedSymbols::ENVCLOSE), Token::EOF]);
    }

    #[test]
    fn matches_left_parenthesis() {
        let input = vec!["(".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LeftParen(ReservedSymbols::INHERITOPEN), Token::EOF]);
    }

    #[test]
    fn matches_right_parenthesis() {
        let input = vec![")".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::RightParen(ReservedSymbols::INHERITCLOSE), Token::EOF]);
    }

    #[test]
    fn matches_comma() {
        let input = vec![",".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Comma, Token::EOF]);
    }

    #[test]
    fn matches_singlequoted_string() {
        let input = vec![
            "'".to_string(),
            "a".to_string(),
            "s".to_string(),
            "d".to_string(),
            "'".to_string()
        ];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("asd")), Token::EOF]);
    }

    #[test]
    fn matches_doublequoted_string() {
        let input = vec![
            "\"".to_string(),
            "a".to_string(),
            "s".to_string(),
            "d".to_string(),
            "\"".to_string()
        ];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("asd")), Token::EOF]);
    }

    #[test]
    fn matches_nested_doublequoted_string() {
        let input = vec![
            "\"".to_string(),
            "'".to_string(),
            "a".to_string(),
            "s".to_string(),
            "d".to_string(),
            "'".to_string(),
            "\"".to_string()
        ];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("'asd'")), Token::EOF])
    }

    #[test]
    fn matches_nested_singlequoted_string() {
        let input = vec![
            "'".to_string(),
            "\"".to_string(),
            "a".to_string(),
            "s".to_string(),
            "d".to_string(),
            "\"".to_string(),
            "'".to_string()
        ];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("\"asd\"")), Token::EOF])
    }

    #[test]
    fn matches_add_operator() {
        let input = vec!["+".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)), Token::EOF]);
    }

    #[test]
    fn matches_subtract_operator() {
        let input = vec!["-".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::SUBTRACT)), Token::EOF]);
    }

    #[test]
    fn matches_multiply_operator() {
        let input = vec!["*".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)), Token::EOF]);
    }

    #[test]
    fn matches_divide_operator() {
        let input = vec!["/".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::DIVIDE)), Token::EOF]);
    }

    #[test]
    fn matches_modulus_operator() {
        let input = vec!["%".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::MODULUS)), Token::EOF]);
    }

    #[test]
    fn matches_exponentiation_operator() {
        let input = vec!["^".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::EXPONENTIATION)), Token::EOF]);
    }

    #[test]
    fn matches_fullstop() {
        let input = vec![".".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::FullStop(OtherOperators::ACCESSOR), Token::EOF]);
    }

    #[test]
    fn matches_assignment_operator() {
        let input = vec!["=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)), Token::EOF]);
    }

    #[test]
    fn matches_digits() {
        let input = vec!["12345".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(Rc::from("12345")), Token::EOF]);
    }

    #[test]
    fn matches_whitespace() {
        let input = vec!["\n".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\n")), Token::EOF]);
    }

    #[test]
    fn matches_identifier() {
        let input = vec!["abc".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("abc")), Token::EOF]);
    }

    #[test]
    fn matches_bool_true() {
        let input = vec!["true".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::TRUE), Token::EOF])
    }

    #[test]
    fn matches_bool_false() {
        let input = vec!["false".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::FALSE), Token::EOF])
    }

    #[test]
    fn matches_keyword_let() {
        let input = vec!["let".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::LET), Token::EOF])
    }

    #[test]
    fn matches_keyword_inherit() {
        let input = vec!["inherit".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::INHERIT), Token::EOF])
    }

    #[test]
    fn matches_keyword_fun() {
        let input = vec!["fun".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::FUN), Token::EOF])
    }

    #[test]
    fn matches_line_terminator() {
        let input = vec![";".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LineTerminator(ReservedSymbols::TERMINATOR), Token::EOF]);
    }

    // Complex token sequence tests
    #[test]
    fn handles_alphabetic_followed_by_number() {
        let input = vec!["a".to_string(), "123".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier(Rc::from("a123")),
            Token::EOF
        ]);
    }

    #[test]
    fn handles_complex_identifier() {
        let input = vec!["abc".to_string(), "-".to_string(), "123".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier(Rc::from("abc-123")),
            Token::EOF
        ]);
    }

    // Tests for edge cases
    #[test]
    fn handles_diacratic_identifier() {
        let input = vec!["üýö".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("üýö")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_strings() {
        let input = vec!["\"".to_string(), "😺".to_string(), "\"".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("😺")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_not_identifiers() {
        let input = vec!["😺".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(tokens.is_err());
    }

    #[test]
    fn handles_windows_newline() {
        let input = vec!["\r\n".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\r\n")), Token::EOF]);
    }

    #[test]
    fn error_peek_beyond_input() {
        let input = vec!["a".to_string()];
        let lexer = Lexer::new(input);
        let result = lexer.peek_n(1);
        assert!(matches!(result, 
            Err(LexerError::IndexOutOfBounds(next, n, len)) 
            if next == 0 && n == 1 && len == 1
        ));
    }

    #[test]
    fn error_peek_with_broken_lexer() {
        let input = vec!["a".to_string()];
        let mut lexer = Lexer::new(input);
        lexer.current = 2;
        let result = lexer.peek_n(0);
        assert!(matches!(result, 
            Err(LexerError::BrokenLexer(pos, len)) if pos == 2 && len == 1
        ));
    }
}