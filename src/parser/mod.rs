mod astnode;
mod error;

use crate::lexer::Token;
use crate::symbols::{Keywords, Booleans, Operators, OtherOperators, ReservedSymbols};
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
    /// Returns an AST node representing the global environment of the program
    /// 
    /// The global environment will contain every element defined in the source code
    /// 
    /// # Errors
    /// Errors are returned as `ParserError` from the parser submethods labelled `parser_`
    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        self.parse_environment(None, None)
    }

    /// Parse an environment
    /// 
    /// Returns an AST node containing an Environment
    /// 
    /// # Errors
    /// Errors are returned as `ParserError` from the parser submethods labelled `parser_`
    fn parse_environment(&mut self, parent: Option<Rc<AstNode>>, name: Option<Rc<str>>) -> Result<AstNode, ParserError> {
        // Thoughts about implementing matching Operator:
        // - It would be better to match against crate::symbol::Operator
        // - However, operators can be either ArithmeticOperators other reserved symbols
        // - There are now two operator enums: ArithmeticOperators and OtherOperators
        // - These now need to be used in the lexer and parser instead of string literals

        let mut current_bindings = Vec::new();
        self.debug_next_token();

        while let Some((pos, token)) = self.advance() {
            self.debug_next_token();
            self.debug_token_tuple(pos, &token);
            match token.borrow() {
                Token::LeftBrace(_) => {
                    // Ignore global braces (global env is constructed by Token::EOF)
                    if parent.is_none() { continue; };
                    let sub_env = self.parse_environment(
                        parent.clone(),
                        None
                    )?;
                    current_bindings.push(Rc::new(sub_env));
                },
                Token::RightBrace(_) => {
                    // Ignore global braces (global env is constructed by Token::EOF)
                    if parent.is_none() { continue; };
                    return Ok(AstNode::Environment {
                        name,
                        bindings: current_bindings,
                        parent,
                        scope: EnvScope::LOCAL
                    })
                },
                Token::Identifier(_) => todo!(),
                Token::FullStop(_) => {
                    let node = self.parse_accession(pos)?;
                    current_bindings.push(Rc::new(node));
                },
                Token::Number(_) => {
                    let node = self.parse_number(pos, &token)?;
                    current_bindings.push(Rc::new(node));
                },
                Token::StringLiteral(string) => {
                    let node = self.parse_string(&string)?;
                    current_bindings.push(Rc::new(node));
                },
                Token::Boolean(Booleans::TRUE) =>
                    current_bindings.push(Rc::new(AstNode::Boolean(true))),
                Token::Boolean(Booleans::FALSE) => 
                    current_bindings.push(Rc::new(AstNode::Boolean(false))),
                Token::Keyword(Keywords::LET) => {
                    let node = self.parse_assignment(parent.clone())?;
                    current_bindings.push(Rc::new(node));
                },
                Token::Keyword(Keywords::INHERIT) => todo!(),
                Token::Keyword(Keywords::FUN) => todo!(),
                Token::Whitespace(ws) =>
                    self.parse_whitespace(ws),
                Token::Operator(op) => {
                    if let Some(prev_operand) = current_bindings.pop() {
                        let node = self.parse_operator(op, &prev_operand)?;
                        current_bindings.push(Rc::new(node));
                    } else {
                        return Err(ParserError::BinaryOpWithNoLHS(pos, self.line));
                    }
                },
                Token::LineTerminator(_) => {
                    if name.is_some() {
                        return Ok(AstNode::Environment {
                            name,
                            bindings: current_bindings,
                            parent,
                            scope: EnvScope::LOCAL
                        });
                    }
                },
                Token::EOF => {
                    // Only valid in the global environment, every other occurrence is an error
                    if parent.is_none() {
                        return Ok(AstNode::Environment {
                            name,
                            bindings: current_bindings,
                            parent,
                            scope: EnvScope::GLOBAL
                        });
                    } else {
                        return Err(ParserError::UnexpectedEOF(pos, self.line));
                    }
                },
            }
        }
        Err(ParserError::UnclosedEnvironment(self.line))
    }

    /// Parse an assignment operation
    /// 
    /// Returns an AST node representing the assignment operation
    /// 
    /// # Errors
    /// Returns an error if the assignment operation is invalid
    fn parse_assignment(&mut self, parent_env: Option<Rc<AstNode>>) -> Result<AstNode, ParserError> {
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Identifier(id) => {
                    return Ok(self.construct_let_statement(&parent_env, id)?);
                },
                _ => return Err(ParserError::MissingLetIdentifier(pos, self.line)), 
            }
        }
        Err(ParserError::ParserLogicError(self.current, self.line))
    }

    fn construct_let_statement(&mut self, parent_env: &Option<Rc<AstNode>>, id: &Rc<str>) -> Result<AstNode, ParserError> {
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Operator(op) => {
                    if *op == Operators::Other(OtherOperators::ASSIGNMENT) {
                        let expr = self.parse_environment(parent_env.clone(), Some(id.clone()))?;
                        return flatten_let_expression(id, expr, pos, self.line, &token);
                    } else {
                        return Err(ParserError::InvalidAssignmentOp(pos, self.line, token.to_string()));
                    }
                },
                _ => return Err(ParserError::MissingAssignmentOp(pos, self.line)),
            }
        }
        Err(ParserError::ParserLogicError(self.current, self.line))
    }
    
    /// Parse an accession operation
    /// 
    /// Returns an AST node representing the accession operation
    /// 
    /// # Errors
    /// Returns an error if the accession operation is invalid
    fn parse_accession(&mut self, pos: usize) -> Result<AstNode, ParserError> {
        if let Some(prev) = self.bindings.pop() {
            match prev.borrow() {
                AstNode::Integer(_) | AstNode::Float(_) => {
                    // We should have caught the FullStop before, so instead we'll throw an error here
                    return Err(ParserError::ParserLogicError(pos, self.line))
                },
                AstNode::Identifier(_) | AstNode::Environment { .. } => {
                    // We're accessing an identifier or environment!
                    return Ok(AstNode::BinaryOp {
                        left: prev.clone(),
                        operator: Operators::Other(OtherOperators::ACCESSOR),
                        right: AstNode::Integer(5).into()
                    })
                },
                _ => {
                    // Syntax error, a full stop does not belong here!
                    return Err(ParserError::InvalidOperation(pos, self.line, Token::FullStop(OtherOperators::ACCESSOR).to_string()))
                },
            }
        } else {
            // Syntax error, source code cannot start with a full stop!
            return Err(ParserError::InvalidOperation(pos, self.line, Token::FullStop(OtherOperators::ACCESSOR).to_string()))
        }
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
        let mut numstr = String::new();

        // Valid numbers start with a number or a full stop (if float)
        match start_token {
            Token::Number(num) => numstr.push_str(num),
            Token::FullStop(_) => numstr.push_str("0."),
            _ => return Err(ParserError::NotANumber(start_pos, self.line, numstr)),
        }
        
        // Numbers are stored as singular tokens from the lexer, so they need to be concatenated first
        while let Some(token) = self.peek() {
            match token {
                Token::Number(num) => {
                    numstr.push_str(num);
                    self.next();
                },
                Token::FullStop(_) => {
                    // A float can only have one decimal point
                    if numstr.contains(".") {
                        return Err(ParserError::MalformedNumber(self.current, self.line, numstr));
                    }
                    numstr.push_str(".");
                    self.next();
                },
                Token::Whitespace(_) =>
                    return Err(ParserError::WhitespaceInNumber(self.current, self.line, numstr)),
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

    /// Debugging function to print the next token in the token queue
    fn debug_next_token(&self) {
        println!("Next token at index: {}, token: {:?}",
            self.current,
            self.peek()
        );
    }

    /// Debugging function to print a pos-token tuple
    fn debug_token_tuple(&self, pos: usize, token: &Token) {
        println!("Grabbed token at index: {}, token: {:?}",
            pos,
            token
        );
    }
}

/// Flatten a let expression into a single let binding
/// 
/// If the let expression contains only one binding, it is flattened into a single let binding
/// 
/// Otherwise, the let expression is returned as is
/// 
/// # Errors
/// Returns an error if the let expression body is empty
fn flatten_let_expression(id: &Rc<str>, expr: AstNode, pos: usize, line: usize, token: &Token) -> Result<AstNode, ParserError> {
    let result: Result<AstNode, ParserError>;
    if let Some(bindings) = expr.get_bindings() {
        if bindings.len() == 1 {
            result = Ok(AstNode::Let {
                name: id.clone(),
                value: bindings[0].clone()
            });
        } else {
            result = Ok(AstNode::Let {
                name: id.clone(),
                value: Rc::new(expr)
            });
        }
    } else {
        result = Err(ParserError::EmptyEnv(pos, line, token.to_string()))
    }
    return result;
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
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Integer(5))], parent: None, scope: EnvScope::GLOBAL });
    }

    #[test]
    fn float() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::FullStop(OtherOperators::ACCESSOR),
            Token::Number("0".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Float(5.0))], parent: None, scope: EnvScope::GLOBAL });
    }

    // TODO: Fix this test: Currently, it errors on meeting a FullStop as the first token (as it should!)
    #[test]
    fn float_with_leading_decimal() {
        let tokens = vec![
            Token::FullStop(OtherOperators::ACCESSOR),
            Token::Number("5".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Float(0.5))], parent: None, scope: EnvScope::GLOBAL });
    }

    #[test]
    fn nested_environments() {
        let tokens = vec![
        Token::LeftBrace(ReservedSymbols::ENVOPEN),
        Token::Number("1".into()),
        Token::RightBrace(ReservedSymbols::ENVCLOSE),
        Token::Number("2".into()),
        Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let AstNode::Environment { bindings, scope, .. } = ast {
            assert_eq!(scope, EnvScope::GLOBAL);
            assert_eq!(bindings.len(), 2);
            // First binding should be a local environment containing 1
            if let AstNode::Environment { bindings: sub_bindings, scope: sub_scope, .. } = &*bindings[0] {
                assert_eq!(sub_scope, &EnvScope::LOCAL);
                assert_eq!(sub_bindings.len(), 1);
                assert_eq!(sub_bindings[0], Rc::new(AstNode::Integer(1)));
            }
            // Second binding should be the number 2
            assert_eq!(&*bindings[1], &AstNode::Integer(2));
        } else {
            panic!("Expected Environment node");
        }
    }

    #[test]
    fn valid_assignment() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "x".into(),
                value: Rc::new(AstNode::Integer(5))
            })],
            parent: None,
            scope: EnvScope::GLOBAL
        });
    }

    // TODO: Fix this test once the error handling is fixed to be more informative
    #[test]
    fn malformed_number() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::FullStop(OtherOperators::ACCESSOR),
            Token::Number("0".into()),
            Token::FullStop(OtherOperators::ACCESSOR),
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

    #[test]
    fn cannot_start_with_fullstop() {
        let tokens = vec![
            Token::FullStop(OtherOperators::ACCESSOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::InvalidOperation(0, 1, ".".to_string()))
    }
}