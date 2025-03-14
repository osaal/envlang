mod astnode;
mod error;

use crate::lexer::Token;
use crate::symbols::ArithmeticOperators;
use crate::symbols::Keywords;
use crate::symbols::Booleans;
use crate::environment::EnvScope;
use std::rc::Rc;
use std::borrow::Borrow;
pub use astnode::AstNode;
pub use error::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            line: 0     // Used for informative errors
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<(usize, Rc<Token>)> {
        if self.current < self.tokens.len() {
            let pos = self.current;
            let ch = Rc::new(self.tokens[self.current].clone());
            self.current += 1;
            return Some((pos, ch));
        } else {
            return None;
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<AstNode, ParserError> {
        let mut bindings = Vec::new();

        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::LeftBrace => todo!(),
                Token::RightBrace => todo!(),
                Token::Identifier(id) => todo!(),
                Token::Number(num) => 
                    bindings.push(self.parse_number(token.as_ref())?.into()),
                Token::StringLiteral(string) =>
                    bindings.push(self.parse_string(&string)?.into()),
                Token::Boolean(Booleans::TRUE) =>
                    bindings.push(AstNode::Boolean(true).into()),
                Token::Boolean(Booleans::FALSE) => 
                    bindings.push(AstNode::Boolean(false).into()),
                Token::Keyword(Keywords::LET) => todo!(),
                Token::Keyword(Keywords::INHERIT) => todo!(),
                Token::Keyword(Keywords::FUN) => todo!(),
                Token::Whitespace(ws) =>
                    self.parse_whitespace(ws),
                Token::Operator(op) => todo!(), // I don't like using an str here...
                Token::FullStop => todo!(),
                Token::EOF => todo!(),
                _ => todo!("Error"),
            }
        }

        Ok(AstNode::Environment {
            name: None,
            bindings,
            parent: None,
            scope: EnvScope::LOCAL
        })
    }

    fn parse_string(&mut self, string: &Rc<str>) -> Result<AstNode, ParserError> {
        Ok(AstNode::String(string.clone()))
    }

    fn parse_whitespace(&mut self, ws: &Rc<str>) {
        // Increase line counter if whitespace is new-line char
        match ws.borrow() {
            "\r\n" | "\n" => self.line += 1,
            _ => (),
        }
    }

    fn parse_operator(&mut self, op: &Rc<str>, prev: &Rc<AstNode>) -> Result<AstNode, ParserError> {
        // Try to parse the next token using self.parse_number()
        // - catch the error! If it errors, it means that whatever came out was not a number-like string
        // If Err(), convert to ParserError::NotANumber (?)
        // If Ok(), check that the element is compatible with prev
        // - If yes, return Ok(AstNode::BinaryOp)
        // - If no, return Err(ParserError::InvalidOperation)
        
        Ok(AstNode::BinaryOp {
            left: AstNode::Integer(5).into(),
            operator: ArithmeticOperators::ADD,
            right: AstNode::Integer(5).into(),
        })
    }

    fn parse_number(&mut self, num: &Token) -> Result<AstNode, ParserError> {
        // Check that num is Token::Number or Token::FullStop
        // If no, return Err(ParserError::NotANumber)
        // If yes, add num to a temporary vector

        // Continue along self.tokens and add all Token::Number or Token::FullStop you meet
        // Once something else is met, finish building the vector
        
        // Try to cast the vector to an isize
        // If possible, return AstNode::Integer(cast_vector)
        // If not, try to cast to an f64
        // If possible, return AstNode::Float(cast_vector)
        // If not, return Err(ParserError::NotANumber)

        Ok(AstNode::Integer(5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}