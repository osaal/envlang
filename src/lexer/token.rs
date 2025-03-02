use crate::symbols::{Booleans, Keywords};

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
