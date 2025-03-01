//! The Envlang lexer
//! 
//! The lexer takes a Unicode-segmented `String` vector from [`segment_graphemes()`] and turns it into a vector of [`Token`]s.
//! 
//! These `Token`s are then intended to be [parsed] into meaningful data structures for the interpreter to evaluate.
//! 
//! [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
//! [`Token`]: ./enum.Token.html
//! [parsed]: ../parser/index.html

use crate::symbols::Keywords;
use crate::symbols::Booleans;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Boolean(Booleans),
    Keyword(Keywords),
    Whitespace(String),
    Operator(String),
    FullStop,
    EOF,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(n) => n.to_string(),
            Token::StringLiteral(s) => format!("\"{}\"", s),
            Token::Boolean(b) => b.to_string(),
            Token::Identifier(i) => i.to_string(),
            Token::Keyword(k) => k.to_string(),
            Token::Operator(o) => o.to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::FullStop => ".".to_string(),
            Token::Whitespace(w) => w.to_string(),
            Token::EOF => "end of file".to_string(),
        }
    }
}

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
    input: Vec<String>,
    pos: usize
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
            input,
            pos: 0
        }
    }

    /// Iterate over the input
    /// 
    /// The function will return the next character in the input
    /// 
    /// It is intended to be used in a while let loop
    fn iterate(&mut self) -> Option<String> {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos].clone();
            self.pos += 1;
            return Some(ch);
        } else {
            return None;
        }
    }

    /// Peek at the Nth input `String`
    /// 
    /// Used as an immutable and flexible alternative to `iterate`
    fn peek_n(&self, n: usize) -> Option<&String> {
        if self.pos > self.input.len() {
            None
        } else if self.pos > n {
            None
        } else if self.input.len() <= n {
            None
        } else {
            Some(&self.input[n])
        }
    }

    /// Peek at following input `String`
    /// 
    /// Used as an immutable alternative to `iterate`
    // fn peek_next(&self) -> Option<&String> {
    //     if self.pos < self.input.len() {
    //         Some(&self.input[self.pos])
    //     } else {
    //         None
    //     }
    // }

    /// Retrieve a selected length input slice
    /// 
    /// Takes the exclusive end of the slice (one more than the position of the ending string)
    /// Returns an immutable reference to the input vector as a slice
    fn get_input_slice(&self, end: usize) -> Option<&[String]> {
        if self.pos > self.input.len() {
            None    // Broken Lexer!
        } else if self.pos > end {
            None    // Would result in inverted vector
        } else if self.input.len() < end {
            None    // Array out of bounds
        } else {
            Some(&self.input[self.pos..end])
        }
    }

    /// Tokenize the input
    /// 
    /// The function will return a vector of `Token` types
    /// 
    /// It iterates over all Strings in the input and matches them to the appropriate `Token` type
    /// 
    /// Strings are handled with a private function `tokenize_string`, and can handle both double- and single-quoted strings (mixing is okay)
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        
        // TODO: Error out if `ch` is not matched
        // Note: self.pos will actually always equal the following element when inside the while-let statement!
        // TODO: Refactor self.pos into self.next or something similar...
        while let Some(unicode_string) = self.iterate() {
            match unicode_string.as_str() {
                "{" => tokens.push(Token::LeftBrace),
                "}" => tokens.push(Token::RightBrace),
                "\"" => tokens.push(self.tokenize_string("\"")),
                "'" => tokens.push(self.tokenize_string("'")),
                "+" | "-" | "*" | "/" | "%" | "^" => tokens.push(Token::Operator(unicode_string.to_string())),
                "." => tokens.push(Token::FullStop),
                unicode_string if unicode_string.chars().all(|c| c.is_ascii_digit()) => {
                    let mut number = unicode_string.to_string();
                    while let Some(next_unicode_string) = self.peek_n(self.pos) {
                        if next_unicode_string.chars().all(|c| c.is_ascii_digit()) {
                            number.push_str(&self.iterate().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
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
                    for following_unicode_string in &self.input[self.pos..] {
                        match following_unicode_string {
                            string if string.chars().all(|c| c.is_alphanumeric()) => {
                                temp.push_str(string);
                            },
                            string if *string == "-".to_string() => {
                                temp.push_str(string);
                            },
                            string if *string == "_".to_string() => {
                                temp.push_str(string);
                            }
                            _ => break,
                        }
                    }

                    match temp {
                        t if t == "let".to_string() => token = Token::Keyword(Keywords::LET),
                        t if t == "inherit".to_string() => token = Token::Keyword(Keywords::INHERIT),
                        t if t == "fun".to_string() => token = Token::Keyword(Keywords::FUN),
                        t if t == "true".to_string() => token = Token::Boolean(Booleans::TRUE),
                        t if t == "false".to_string() => token = Token::Boolean(Booleans::FALSE),
                        _ => token = Token::Identifier(temp),
                    }
                    tokens.push(token);
                },
                unicode_string if unicode_string.chars().all(|c| c.is_whitespace()) => tokens.push(Token::Whitespace(unicode_string.to_string())),
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
            if ch != matched {
                value.push_str(&ch);
            } else {
                break;
            }
        }

        return Token::StringLiteral(value);
    }
}

// Unit tests for lexer.rs
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(tokens, vec![Token::StringLiteral("asd".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::StringLiteral("asd".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::StringLiteral("'asd'".to_string()), Token::EOF])
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
        assert_eq!(tokens, vec![Token::StringLiteral("\"asd\"".to_string()), Token::EOF])
    }

    #[test]
    fn matches_add_operator() {
        let input = vec!["+".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("+".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_subtract_operator() {
        let input = vec!["-".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("-".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_multiply_operator() {
        let input = vec!["*".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("*".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_divide_operator() {
        let input = vec!["/".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("/".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_modulus_operator() {
        let input = vec!["%".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("%".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_exponentiation_operator() {
        let input = vec!["^".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Operator("^".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::Number("12345".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_whitespace() {
        let input = vec!["\n".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Whitespace("\n".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_identifier() {
        let input = vec!["abc".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::Identifier("abc".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::Identifier("üýö".to_string()), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_strings() {
        let input = vec!["\"".to_string(), "😺".to_string(), "\"".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral("😺".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::Whitespace("\r\n".to_string()), Token::EOF]);
    }
}