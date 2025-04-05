use crate::symbols::{Booleans, Keywords, Operators};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Identifier(Rc<str>),
    Number(Rc<str>),
    StringLiteral(Rc<str>),
    Boolean(Booleans),
    Keyword(Keywords),
    Whitespace(Rc<str>),
    Operator(Operators),
    LineTerminator,
    Comma,
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
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::Whitespace(w) => w.to_string(),
            Token::EOF => "EOF".to_string(),
            Token::LineTerminator => ";".to_string(),
            Token::Comma => ",".to_string(),
        }
    }
}
