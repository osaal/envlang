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
    /// The input is expected to be a Unicode grapheme-segmented vector of strings
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
    /// # Errors
    /// 
    /// Returns:
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
    /// Takes the exclusive end of the slice (one more than the position of the ending string)
    /// Returns a slice of `Rc<str>`s
    /// 
    /// # Errors
    /// 
    /// Returns:
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
    /// The function will return a vector of `Token` types
    /// 
    /// It iterates over all `Rc<str>`s in the input and matches them to the appropriate `Token` type
    /// 
    /// Strings are handled with a private function `tokenize_string`, and can handle both double- and single-quoted strings (mixing is okay)
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        
        // TODO: Error out if `ch` is not matched
        // Note: self.pos will actually always equal the following element when inside the while-let statement!
        while let Some(unicode_string) = self.iterate() {
            match unicode_string.borrow() {
                "{" => tokens.push(Token::LeftBrace),
                "}" => tokens.push(Token::RightBrace),
                "\"" => tokens.push(self.tokenize_string("\"")),
                "'" => tokens.push(self.tokenize_string("'")),
                "+" | "-" | "*" | "/" | "%" | "^" => tokens.push(Token::Operator(unicode_string)),
                "." => tokens.push(Token::FullStop),
                unicode_string if unicode_string.chars().all(|c| c.is_ascii_digit()) => {
                    let mut number = unicode_string.to_string();
                    // We loop until 1) non-digit character, 2) end of input (IndexOutOfBounds error), 3) Invalid lexing state (BrokenLexer error)
                    while let Ok(next_unicode_string) = self.peek_n(self.next) {
                        if next_unicode_string.chars().all(|c| c.is_ascii_digit()) {
                            // Safe to unwrap because of the error handled in the while let loop condition
                            number.push_str(&self.iterate().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(Rc::from(number)));
                },
                unicode_string if unicode_string.chars().all(|c| c.is_alphabetic()) => {
                    // Takes alphabetic input and constructs one of three potential options:
                    // 1. A boolean value ("true", "false")
                    // 2. A keyword (see symbols::Keywords)
                    // 3. An identifier (a sequence of alphanumeric characters or one of symbols::GenericSymbol)
                    // TODO: Since the parent call to .all() returns true on an empty iterator,
                    // make sure to return an error in this case -- otherwise empty strings become Identifiers!

                    let mut temp = String::new();
                    let token: Token;

                    // We should error if unicode_string was empty, since it's a valid condition but an invalid identifier

                    // Push the first character onto the stack before starting iteration
                    temp.push_str(&unicode_string);

                    // The set of valid symbols is that of an identifier, with booleans and keywords having subsets of this
                    for following_unicode_string in &self.input[self.next..] {
                        match following_unicode_string.as_ref() {
                            s if s.chars().all(|c| c.is_alphanumeric()) => temp.push_str(following_unicode_string),
                            "-" => temp.push_str(following_unicode_string),
                            "_" => temp.push_str(following_unicode_string),
                            _ => break,
                        }
                    }

                    match temp {
                        t if t == "let".to_string() => token = Token::Keyword(Keywords::LET),
                        t if t == "inherit".to_string() => token = Token::Keyword(Keywords::INHERIT),
                        t if t == "fun".to_string() => token = Token::Keyword(Keywords::FUN),
                        t if t == "true".to_string() => token = Token::Boolean(Booleans::TRUE),
                        t if t == "false".to_string() => token = Token::Boolean(Booleans::FALSE),
                        _ => token = Token::Identifier(Rc::from(temp)),
                    }
                    tokens.push(token);
                },
                unicode_string if unicode_string.chars().all(|c| c.is_whitespace()) =>
                    tokens.push(Token::Whitespace(Rc::from(unicode_string))),
                _ => {}, // TODO: Throw an appropriate error
            }
        }

        tokens.push(Token::EOF);
        return tokens;
    }

    /// Tokenize a string
    /// 
    /// The function tokenizes a string between matching start and end delimiters as given in `matched`
    /// 
    /// The function will continue to iterate until the given closing delimiter is found
    fn tokenize_string(&mut self, matched: &str) -> Token {
        let mut value: String = String::new();

        while let Some(ch) = self.iterate() {
            if *ch != *matched {
                value.push_str(&ch);
            } else {
                break;
            }
        }

        return Token::StringLiteral(Rc::from(value));
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