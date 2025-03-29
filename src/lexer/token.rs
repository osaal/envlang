use crate::symbols::{Booleans, Keywords, Operators, OtherOperators, ReservedSymbols};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LeftBrace(ReservedSymbols),
    RightBrace(ReservedSymbols),
    LeftParen(ReservedSymbols),
    RightParen(ReservedSymbols),
    LeftBracket(ReservedSymbols),
    RightBracket(ReservedSymbols),
    Identifier(Rc<str>),
    Number(Rc<str>),
    StringLiteral(Rc<str>),
    Boolean(Booleans),
    Keyword(Keywords),
    Whitespace(Rc<str>),
    Operator(Operators),
    LineTerminator(ReservedSymbols),
    FullStop(OtherOperators),
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
            Token::LeftBrace(b)
            | Token::RightBrace(b)
            | Token::LeftParen(b)
            | Token::RightParen(b)
            | Token::LeftBracket(b)
            | Token::RightBracket(b) => b.to_string(),
            Token::FullStop(fs) => fs.to_string(),
            Token::Whitespace(w) => w.to_string(),
            Token::EOF => "end of file".to_string(),
            Token::LineTerminator(lt) => lt.to_string(),
            Token::Comma => ",".to_string(),
        }
    }
}
