//! The Envlang lexer
//! 
//! The lexer takes a Unicode-segmented `String` vector from [`segment_graphemes()`] and turns it into a vector of [`Token`]s.
//! 
//! These `Token`s are then intended to be [parsed] into an Abstract Syntax Tree.
//! 
//! [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
//! [`Token`]: ./enum.Token.html
//! [parsed]: ../parser/index.html

mod token;
mod error;

pub use token::Token;
pub use error::LexerError;

use crate::symbols::{Keywords, Booleans};
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
/// 
/// The Lexer never errors. Instead, if it cannot lex an input `String`, it skips the `String`. This may be changed in the future.
/// 
/// [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
pub struct Lexer {
    input: Vec<Rc<str>>,
    next: usize
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
            next: 0
        }
    }

    /// Iterate over the input
    /// 
    /// The function will return the next character in the input
    /// 
    /// It is intended to be used in a while let loop
    fn iterate(&mut self) -> Option<Rc<str>> {
        if self.next < self.input.len() {
            let ch = Rc::clone(&self.input[self.next]);
            self.next += 1;
            return Some(ch);
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
        let next: usize = self.next;

        if next > length {
            Err(LexerError::BrokenLexer(next, length))
        } else if n >= length {
            Err(LexerError::IndexOutOfBounds(next, n, length))
        } else {
            Ok(Rc::clone(&self.input[n]))
        }
    }

    /// Retrieve a selected length input slice
    /// 
    /// Gets a slice from the current position to the given end parameter
    /// 
    /// # Arguments
    /// * `end` - The exclusive end of the slice
    /// 
    /// # Returns
    /// A Result containing a slice of input `Rc<str>`s
    /// 
    /// # Errors
    /// - `LexerError::BrokenLexer` if the currently-held position is beyond input length
    /// - `LexerError::InvertedSlice` if the start position is greater than the end
    /// - `LexerError::SliceOutOfBounds` if the end position is beyond input length
    fn get_input_slice(&self, end: usize) -> Result<&[Rc<str>], LexerError> {
        let length: usize = self.input.len();
        let next: usize = self.next;

        if next > length {
            Err(LexerError::BrokenLexer(next, length))
        } else if next > end {
            Err(LexerError::InvertedSlice(next, end))
        } else if length < end {
            Err(LexerError::SliceOutOfBounds(next, end, length))
        } else {
            Ok(&self.input[next..end])
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
        
        while let Some(unicode_string) = self.iterate() {
            match unicode_string.borrow() {
                "{" =>
                    tokens.push(Token::LeftBrace),
                "}" =>
                    tokens.push(Token::RightBrace),
                "\"" =>
                    tokens.push(self.tokenize_string("\"")?),
                "'" =>
                    tokens.push(self.tokenize_string("'")?),
                "+" | "-" | "*" | "/" | "%" | "^" =>
                    tokens.push(Token::Operator(unicode_string)),
                "." =>
                    tokens.push(Token::FullStop),
                unicode_string if unicode_string.chars().all(|c| c.is_ascii_digit()) =>
                    tokens.push(self.tokenize_number(unicode_string)?),
                unicode_string if unicode_string.chars().all(|c| c.is_alphabetic()) =>
                    tokens.push(self.tokenize_alphabetics(unicode_string)?),
                unicode_string if unicode_string.chars().all(|c| c.is_whitespace()) =>
                    tokens.push(Token::Whitespace(Rc::from(unicode_string))),
                _ => return Err(LexerError::UnrecognizedInput(self.next, unicode_string.to_string()))
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
    fn tokenize_number(&mut self, unicode_string: &str) -> Result<Token, LexerError> {
        let mut number = unicode_string.to_string();
        
        // Validate first character to exit early in case of inappropriate input
        if !unicode_string.chars().all(|c| c.is_ascii_digit()) {
            return Err(LexerError::InvalidToken(self.next, unicode_string.to_string()));
            // TODO: Write test to check this error's pos value, in case we are off-by-one
        }

        number.push_str(unicode_string);

        // We exit out in three conditions:
        // 1. Hitting a non-digit character (valid termination, no error)
        // 2. Reaching end of input (valid termination, no error)
        // 3. Invalid lexer state (caught by the while-let loop, no error)
        while let Ok(next_unicode_string) = self.peek_n(self.next) {
            if !next_unicode_string.chars().all(|c| c.is_ascii_digit()) {
                break;
            }
            number.push_str(&self.iterate().unwrap());
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
    fn tokenize_alphabetics(&mut self, unicode_string: &str) -> Result<Token, LexerError> {
        let mut temp = String::new();

        if unicode_string == "" {
            return Err(LexerError::EmptyIdentifier(self.next));
        }

        // Doesn't start with an alphabetic character == invalid identifier/boolean/keyword
        if !unicode_string.chars().all(|c| c.is_alphabetic()) {
            return Err(LexerError::InvalidToken(self.next, unicode_string.to_string()));
        }

        temp.push_str(unicode_string); // Push the first character onto the stack before starting iteration
        
        // The set of valid symbols is that of an identifier, with booleans and keywords having subsets of identifiers
        while let Ok(following_unicode_string) = self.peek_n(self.next) {
            match following_unicode_string.as_ref() {
                s if s.chars().all(|c| c.is_alphanumeric()) ||
                    s == "-" ||
                    s == "_" => {
                        temp.push_str(&self.iterate().unwrap());
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
    fn tokenize_string(&mut self, matched: &str) -> Result<Token, LexerError> {
        let mut value: String = String::new();
        let start_pos = self.next;

        loop {
            match self.peek_n(self.next) {
                Ok(ch) => {
                    if ch.as_ref() != matched {
                        value.push_str(&self.iterate().unwrap());
                    } else {
                        self.iterate(); // Skip over the closing brace
                        break;
                    }
                },
                Err(LexerError::IndexOutOfBounds(_, _, _)) |
                Err(LexerError::BrokenLexer(_, _)) => {
                    return Err(LexerError::UnterminatedString(start_pos, value))
                },
                Err(e) => return Err(e),
            }
        }

        Ok(Token::StringLiteral(Rc::from(value)))
    }
}

// Unit tests for lexer.rs
#[cfg(test)]
mod tests {
    use super::*;

    // Tests for error types
    #[test]
    fn error_input_slice_broken_lexer() {
        let input = vec!["a".to_string(), "b".to_string()];
        let mut lexer = Lexer::new(input);

        // Break the lexer position manually
        lexer.next = 3;

        let result = lexer.get_input_slice(2);

        assert!(matches!(
            result,
            Err(LexerError::BrokenLexer(pos, len)) if pos == 3 && len == 2
        ));
    }

    #[test]
    fn error_input_slice_inverted_slice() {
        let input = vec!["a".to_string(), "b".to_string()];
        let mut lexer = Lexer::new(input);
        
        // Move position to 2, try to get slice to position 1
        lexer.next = 2;
        
        let result = lexer.get_input_slice(1);
        assert!(matches!(result, 
            Err(LexerError::InvertedSlice(start, end)) if start == 2 && end == 1
        ));
    }

    #[test]
    fn error_input_slice_out_of_bounds() {
        let input = vec!["a".to_string(), "b".to_string()];
        let lexer = Lexer::new(input);
        
        // Try to get slice beyond input length
        let result = lexer.get_input_slice(3);
        assert!(matches!(result, 
            Err(LexerError::SliceOutOfBounds(pos, end, len)) if pos == 0 && end == 3 && len == 2
        ));
    }

    #[test]
    fn input_slice_success() {
        let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let lexer = Lexer::new(input);
        
        let result = lexer.get_input_slice(2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    // Tests for correct behaviour
    #[test]
    fn matches_left_brace() {
        let input = vec!["{".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::LeftBrace, Token::EOF]);
    }

    #[test]
    fn matches_right_brace() {
        let input = vec!["}".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::RightBrace, Token::EOF]);
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
        let tokens = Lexer::new(input).tokenize();
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
        let tokens = Lexer::new(input).tokenize();
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
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("asd")), Token::EOF])
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
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("\"asd\"")), Token::EOF])
    }

    #[test]
    fn matches_add_operator() {
        let input = vec!["+".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("+")), Token::EOF]);
    }

    #[test]
    fn matches_subtract_operator() {
        let input = vec!["-".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("-")), Token::EOF]);
    }

    #[test]
    fn matches_multiply_operator() {
        let input = vec!["*".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("*")), Token::EOF]);
    }

    #[test]
    fn matches_divide_operator() {
        let input = vec!["/".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("/")), Token::EOF]);
    }

    #[test]
    fn matches_modulus_operator() {
        let input = vec!["%".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("%")), Token::EOF]);
    }

    #[test]
    fn matches_exponentiation_operator() {
        let input = vec!["^".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator(Rc::from("^")), Token::EOF]);
    }

    #[test]
    fn matches_fullstop() {
        let input = vec![".".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::FullStop, Token::EOF]);
    }

    #[test]
    fn matches_digits() {
        let input = vec!["12345".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Number(Rc::from("12345")), Token::EOF]);
    }

    #[test]
    fn matches_whitespace() {
        let input = vec!["\n".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\n")), Token::EOF]);
    }

    #[test]
    fn matches_identifier() {
        let input = vec!["abc".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("abc")), Token::EOF]);
    }

    #[test]
    fn matches_bool_true() {
        let input = vec!["true".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::TRUE), Token::EOF])
    }

    #[test]
    fn matches_bool_false() {
        let input = vec!["false".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::FALSE), Token::EOF])
    }

    #[test]
    fn matches_keyword_let() {
        let input = vec!["let".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::LET), Token::EOF])
    }

    #[test]
    fn matches_keyword_inherit() {
        let input = vec!["inherit".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::INHERIT), Token::EOF])
    }

    #[test]
    fn matches_keyword_fun() {
        let input = vec!["fun".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::FUN), Token::EOF])
    }

    // Tests for edge cases
    #[test]
    fn handles_diacratic_identifier() {
        let input = vec!["üýö".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("üýö")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_strings() {
        let input = vec!["\"".to_string(), "😺".to_string(), "\"".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("😺")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_not_identifiers() {
        let input = vec!["😺".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn handles_windows_newline() {
        let input = vec!["\r\n".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\r\n")), Token::EOF]);
    }
}