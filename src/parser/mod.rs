mod astnode;
mod error;

use crate::lexer::Token;
use crate::symbols::{Keywords, Booleans, Operators};
use crate::environment::EnvScope;
use std::rc::Rc;
use std::borrow::Borrow;

pub use astnode::AstNode;
pub use error::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    bindings: Vec<Rc<AstNode>>,
    current: usize,
    line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            bindings: Vec::new(),
            current: 0,
            line: 1     // Used for informative errors
        }
    }

    /// Get the current token in queue
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Increment the current token index by 1
    fn next(&mut self) {
        self.current += 1;
    }

    /// Get current token and advance the index by 1
    /// 
    /// Returns the current token and its position in the token vector
    /// 
    /// If the current token is the last token in the vector, returns None
    fn advance(&mut self) -> Option<(usize, Rc<Token>)> {
        if self.current < self.tokens.len() {
            let pos = self.current;
            let ch = Rc::new(self.peek().unwrap().clone()); // Safe to unwrap because we checked the length
            self.next();
            return Some((pos, ch));
        } else {
            return None;
        }
    }

    /// Parse the tokens into an AST
    /// 
    /// Returns an AST node representing the program
    /// 
    /// Simple wrapper for the `parse_program` internal method
    /// 
    /// # Errors
    /// Errors are returned as `ParserError` from the parser submethods labelled `parser_`
    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        self.parse_program()
    }

    /// Parse the program
    /// 
    /// Returns an AST node containing an Environment representing the program
    /// 
    /// # Errors
    /// Errors are returned as `ParserError` from the parser submethods labelled `parser_`
    fn parse_program(&mut self) -> Result<AstNode, ParserError> {
        // Thoughts about implementing matching Operator:
        // - It would be better to match against crate::symbol::Operator
        // - However, operators can be either ArithmeticOperators other reserved symbols
        // - There are now two operator enums: ArithmeticOperators and OtherOperators
        // - These now need to be used in the lexer and parser instead of string literals

        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::LeftBrace => todo!(),
                Token::RightBrace => todo!(),
                Token::Identifier(_) => todo!(),
                Token::FullStop => {
                    if let Some(prev) = self.bindings.pop() {
                        // Check type of AstNode
                        // If it is an identifier, FullStop represents the accessor operation
                        match prev.borrow() {
                            AstNode::Integer(_) | AstNode::Float(_) => {
                                // We're building a number!
                                // However, this should have been caught by Token::Number in a previous step...
                            },
                            AstNode::Identifier(_) | AstNode::Environment { .. } => {
                                // We're accessing an identifier or environment!
                                // This will not have been caught by any match arm or sub-method yet
                            },
                            _ => {
                                // Syntax error, a full stop does not belong here!
                            },
                        }
                    } else {
                        // Syntax error, a full stop does not belong at the start of the source code!
                    }
                },
                Token::Number(_) => { // FullStop is caught before and sent to number parsing if syntactically correct
                    let temp = self.parse_number(pos, &token)?.into();
                    self.bindings.push(temp);
                },
                Token::StringLiteral(string) => {
                    let temp = self.parse_string(&string)?.into();
                    self.bindings.push(temp);
                },
                Token::Boolean(Booleans::TRUE) =>
                    self.bindings.push(AstNode::Boolean(true).into()),
                Token::Boolean(Booleans::FALSE) => 
                    self.bindings.push(AstNode::Boolean(false).into()),
                Token::Keyword(Keywords::LET) => todo!(),
                Token::Keyword(Keywords::INHERIT) => todo!(),
                Token::Keyword(Keywords::FUN) => todo!(),
                Token::Whitespace(ws) =>
                    self.parse_whitespace(ws),
                Token::Operator(op) => {
                    if let Some(prev_operand) = self.bindings.pop() {
                        let temp = self.parse_operator(op, &prev_operand)?.into();
                        self.bindings.push(temp);
                    } else {
                        return Err(ParserError::BinaryOpWithNoLHS(pos, self.line));
                    }
                },
                Token::EOF => (),
            }
        }

        Ok(AstNode::Environment {
            name: None,
            bindings: self.bindings.clone(),
            parent: None,
            scope: EnvScope::LOCAL
        })
    }

    /// Parse a string literal
    /// 
    /// Returns an AST node containing the string literal
    fn parse_string(&mut self, string: &Rc<str>) -> Result<AstNode, ParserError> {
        Ok(AstNode::String(string.clone()))
    }

    /// Parse a whitespace token
    /// 
    /// Increments the line counter if the whitespace is a new-line character
    /// 
    /// In other cases, does nothing
    fn parse_whitespace(&mut self, ws: &Rc<str>) {
        // Increase line counter if whitespace is new-line char
        match ws.borrow() {
            "\r\n" | "\n" => self.line += 1,
            _ => (),
        }
    }

    fn parse_operator(&mut self, op: &Operators, prev: &Rc<AstNode>) -> Result<AstNode, ParserError> {
        // Here are (some) valid operations that need to be covered:
        // 1. String + {string, identifier representing string} (concatenation)
        // 2. Integer +-*^% {integer, float} (convert LHS to float if RHS is float)
        // 3. Integer / {integer, float} (convert LHS to float in all cases)
        // 4. Float +-*^%/ {integer, float} (no conversion needed)
        // 5. Identifier = {expression, left brace, identifier} (parse RHS and add to identifier)
        // 6. Identifier.identifier (accession)

        // TEMP: Remove once build is secured
        Ok(AstNode::BinaryOp {
            left: prev.clone(),
            operator: op.clone(),
            right: AstNode::Integer(5).into(),
        })
    }

    fn parse_number(&mut self, start_pos: usize, start_token: &Token) -> Result<AstNode, ParserError> {
        // We can enter here from two places:
        // 1. Matching a FullStop, and the previous AstToken was an integer or float (this should never happen... right?)
        // - Here, we need to convert the previous AstToken's value into a String, concatenate it with a FullStop, and continue parsing
        // 2. Matching a Token::Number
        // - Here, we need to do regular parsing (step until second FullStop (error) or first non-Number (ok))

        let mut numstr = String::new();

        // Valid numbers start with a number or a full stop (if float)
        match start_token {
            Token::Number(num) => numstr.push_str(num),
            Token::FullStop => numstr.push_str("0."),
            _ => return Err(ParserError::NotANumber(start_pos, self.line, numstr)),
        }
        
        // Numbers are stored as singular tokens from the lexer, so they need to be concatenated first
        while let Some(token) = self.peek() {
            match token {
                Token::Number(num) => {
                    numstr.push_str(num);
                    self.next();
                },
                Token::FullStop => {
                    // A float can only have one decimal point
                    if numstr.contains(".") {
                        return Err(ParserError::MalformedNumber(self.current, self.line, numstr));
                    }

                    numstr.push_str(".");
                    self.next();
                },
                _ => break,
            }
        }
        
        // Try to cast the vector to an isize
        // If possible, return AstNode::Integer(cast_vector); If not, try to cast to an f64
        // If possible, return AstNode::Float(cast_vector); If not, return Err(ParserError::NotANumber)
        numstr.parse::<isize>()
            .map(|num| AstNode::Integer(num))
            .or_else(|_| numstr.parse::<f64>().map(|num| AstNode::Float(num)))
            .map_err(|_| ParserError::NotANumber(self.current, self.line, numstr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Integer(5))], parent: None, scope: EnvScope::LOCAL });
    }

    #[test]
    fn float() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::FullStop,
            Token::Number("0".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Float(5.0))], parent: None, scope: EnvScope::LOCAL });
    }

    #[test]
    fn float_with_leading_decimal() {
        let tokens = vec![
            Token::FullStop,
            Token::Number("5".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Float(0.5))], parent: None, scope: EnvScope::LOCAL });
    }

    // TODO: Fix this test once the error handling is fixed to be more informative
    #[test]
    fn malformed_number() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::FullStop,
            Token::Number("0".into()),
            Token::FullStop,
            Token::Number("0".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::MalformedNumber(3, 1, "5.0".into()));
    }

    #[test]
    fn not_a_number() {
        let tokens = vec![
            Token::Number("abc".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::NotANumber(1, 1, "abc".into()));
    }
}