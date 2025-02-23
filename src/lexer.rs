//! The Envlang lexer
//! 
//! The lexer takes a Unicode-segmented `String` vector from [`segment_graphemes()`] and turns it into a vector of [`Token`]s.
//! 
//! These `Token`s are then intended to be [parsed] into meaningful data structures for the interpreter to evaluate.
//! 
//! [`segment_graphemes()`]: ../unicodesegmenters/fn.segment_graphemes.html
//! [`Token`]: ./enum.Token.html
//! [parsed]: ../parser/index.html

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Whitespace(String),
    Operator(String),
    FullStop,
    EOF,
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

    /// Peek at following input `String`
    /// 
    /// Used as an immutable alternative to `iterate`
    fn peek_next(&self) -> Option<&String> {
        if self.pos < self.input.len() {
            Some(&self.input[self.pos])
        } else {
            None
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
        while let Some(ch) = self.iterate() {
            match ch.as_str() {
                "{" => tokens.push(Token::LeftBrace),
                "}" => tokens.push(Token::RightBrace),
                "\"" => tokens.push(self.tokenize_string("\"")),
                "'" => tokens.push(self.tokenize_string("'")),
                "+" | "-" | "*" | "/" | "%" | "^" => tokens.push(Token::Operator(ch.to_string())),
                "." => tokens.push(Token::FullStop),
                ch if ch.chars().all(|c| c.is_ascii_digit()) => {
                    let mut number = ch.to_string();
                    while let Some(next_ch) = self.peek_next() {
                        if next_ch.chars().all(|c| c.is_ascii_digit()) {
                            number.push_str(&self.iterate().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
                },
                ch if ch.chars().all(|c| c.is_alphabetic()) => tokens.push(Token::Identifier(ch.to_string())),
                ch if ch.chars().all(|c| c.is_whitespace()) => tokens.push(Token::Whitespace(ch.to_string())),
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
        value.push_str(matched);

        while let Some(ch) = self.iterate() {
            value.push_str(&ch);
            if ch == matched {
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
        let input = vec!["'".to_string(), "a".to_string(), "s".to_string(), "d".to_string(), "'".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral("'asd'".to_string()), Token::EOF]);
    }

    #[test]
    fn matches_doublequoted_string() {
        let input = vec!["\"".to_string(), "a".to_string(), "s".to_string(), "d".to_string(), "\"".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::StringLiteral("\"asd\"".to_string()), Token::EOF]);
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
        assert_eq!(tokens, vec![Token::StringLiteral("\"😺\"".to_string()), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_not_identifiers() {
        let input = vec!["😺".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![Token::EOF]);
    }
}