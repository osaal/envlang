//! The Envlang parser
//! 
//! The parser takes a lexed [`Token`] vector from the lexer and turns it into an Abstract Syntax Tree of `AstNode`s.
//! 
//! This AST is then intended to be validated.
//! 
//! # Documentation notes
//! 
//! Unless specified, phrases such as "followed by", "enclosed by", "surrounded by", etc., allow for unlimited whitespace around the syntactical object in question.
//! 
//! # Error handling
//! 
//! The parser provides numerous different error types enumerated in [`ParserError`].
//! 
//! All errors include position information for reporting.
//! 
//! [`Token`]: ../lexer/enum.Token.html
//! [`ParserError`]: ./enum.ParserError.html

mod astnode;
mod error;

pub use astnode::AstNode;
pub use error::ParserError;

use crate::lexer::Token;
use crate::symbols::{Keywords, Booleans, Operators, ArithmeticOperators, OtherOperators};
use std::rc::Rc;
use std::borrow::Borrow;

/// The `ParseContext` enum represents the context in which the parser is operating at any given time.
/// 
/// The context changes how the parser handles some special tokens, e.g., the EOF token.
#[derive(Debug, PartialEq, Clone)]
enum ParseContext {
    Normal,
    Operation,
    Function,
    FunctionReturn,
}

/// The `Parser` struct holds the [`Token`] vector from the lexer, as well as the index of the currently lexed token and the line number.
/// 
/// The line number is calculated from the amount of recognised line-breaks, and is one-indexed.
/// 
/// # Panics
/// 
/// The parser does not panic, as it instead converts all invalid states into [`ParserError`] objects.
/// 
/// # Errors
/// 
/// All parser methods return `Result<AstNode, ParserError>` types. The errors contain a human-readable description of the intended use, as well as information on:
/// - The token index where the error was triggered.
/// - The error-triggering token (in most cases).
/// - The line number from the input source code.
/// 
/// [`Token`]: ../lexer/enum.Token.html
/// [`ParserError`]: ./enum.ParserError.html
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    line: usize,
}

impl Parser {
    /// Initializes a new Parser with a given input token vector.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            line: 1
        }
    }

    /// Gets the `current` [`Token`] in queue.
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.current) }

    /// Increments the `current` [`Token`] index.
    fn next(&mut self) { self.current += 1; }

    /// Gets current [`Token`] and increment the `current` Token index.
    /// 
    /// Returns `None` if the `current` token is the last token in the vector.
    fn advance(&mut self) -> Option<(usize, Rc<Token>)> {
        if self.current < self.tokens.len() {
            let pos = self.current;
            // Safety: Length is guaranteed to be valid
            let ch = Rc::new(self.peek().unwrap().clone());
            self.next();
            return Some((pos, ch));
        } else {
            return None;
        }
    }

    /// Parses the input into an [`AstNode`] representing the global environment of the program.
    /// 
    /// The global environment will contain every element defined in the source code.
    /// 
    /// The parsing context is always set to [`ParseContext::Normal`].
    /// 
    /// # Errors
    /// Errors are returned as [`ParserError`] from the parser submethods
    pub fn parse(&mut self) -> Result<AstNode, ParserError> { self.parse_environment(None, None, ParseContext::Normal) }

    /// Returns an [`AstNode::Environment`] representing an environment.
    /// 
    /// # Arguments
    /// * `parent`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// * `name`: An `Option`al reference-counted pointer to the name of the environment (as an `str`), or `None` for an unnamed environment.
    /// * `context`: The [`ParseContext`] within which the environment is being parsed.
    /// 
    /// # The EOF token
    /// 
    /// The end-of-file token [`Token::EOF`] is used to mark the finished parsing of a complete program.
    /// 
    /// The token is only a valid final token in two contexts:
    /// * A return statement (as this implies that the final element in the global environment was a function declaration, which is valid albeit useless.)
    /// * The global environment
    /// 
    /// In all other contexts, meeting the EOF token represents some form of syntax error.
    /// 
    /// However, since the type of syntax error cannot easily be ascertained, the EOF token is treated as a generic error (see below).
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_environment`](Parser::parse_environment), [`parse_identifier`](Parser::parse_identifier), [`parse_number`](Parser::parse_number), [`parse_string`](Parser::parse_string), [`parse_assignment`](Parser::parse_assignment), and [`parse_operator`](Parser::parse_environment).
    /// * [`ParserError::UnexpectedReturn`]: The return keyword was used in a non-function context.
    /// * [`ParserError::BinaryOpWithNoLHS`]: A binary operation lacked a left-hand side. This may occur with arithmetic operations or accessions.
    /// * [`ParserError::UnexpectedEOF`]: A non-global normal environment or an operation encountered the EOF token.
    /// * [`ParserError::MissingReturnStatement`]: A function environment encountered the EOF token.
    /// * [`ParserError::UnclosedEnvironment`]: EOF token was consumed before a non-global, non-function-return environment finished parsing.
    fn parse_environment(&mut self, parent: Option<Rc<AstNode>>, name: Option<Rc<str>>, context: ParseContext) -> Result<AstNode, ParserError> {
        // Create a temporary environment to handle parentage
        let mut current_env: AstNode = AstNode::Environment {
            name: name.clone(),
            bindings: Vec::new(),
            parent: parent.clone(),
        };

        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::LeftBrace(_) => {
                    // Ignore extra left brace in the global environment
                    if parent.is_none() { continue; };

                    // Create a sub-environment if `ParseContext::Normal`
                    if context == ParseContext::Normal {
                        let sub_env: AstNode = self.parse_environment(
                            Some(Rc::new(current_env.clone())),
                            None,
                            ParseContext::Normal
                        )?;
                        if let AstNode::Environment { ref mut bindings, .. } = current_env {
                            bindings.push(Rc::new(sub_env));
                        }
                    } else {
                        continue;
                    }
                },
                Token::RightBrace(_) => {
                    // Ignore extra right brace in the global environment
                    if parent.is_none() { continue; };
                    return Ok(current_env);
                },
                Token::LeftParen(_) => continue,        // Covered by parse_inherit_clause
                Token::RightParen(_) => continue,       // Covered by parse_inherit_clause
                Token::Comma => continue,               // Covered by parse_inherit_clause and parse_function_clause
                Token::LeftBracket(_) => continue,      // Covered by parse_function_clause
                Token::RightBracket(_) => continue,     // Covered by parse_function_clause
                Token::Keyword(Keywords::INHERIT) => {  // Covered by construct_let_statement
                    continue;
                },
                Token::Keyword(Keywords::FUN) => {      // Covered by parse_assignment
                    continue;
                },
                Token::Identifier(id) => {
                    let node: AstNode = self.parse_identifier(&id)?;
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(node));
                    }
                },
                Token::Number(_) => {
                    let node: AstNode = self.parse_number(pos, &token)?;
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(node));
                    }
                },
                Token::StringLiteral(string) => {
                    let node: AstNode = self.parse_string(&string)?;
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(node));
                    }
                },
                Token::Boolean(Booleans::TRUE) => {
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(AstNode::Boolean(true)));
                    }
                },
                Token::Boolean(Booleans::FALSE) => {
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(AstNode::Boolean(false)));
                    }
                },
                Token::Keyword(Keywords::LET) => {
                    let node: AstNode = self.parse_assignment(Some(Rc::new(current_env.clone())))?;
                    if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.push(Rc::new(node));
                    }
                },
                Token::Keyword(Keywords::RETURN) =>
                    // Return keyword is only valid in ParseContext::Function
                    match context {
                        ParseContext::Function => {
                            let return_env = self.parse_environment(
                                parent.clone(),
                                None,
                                ParseContext::FunctionReturn
                            )?;

                            if let AstNode::Environment { ref mut bindings, .. } = current_env {
                                bindings.push(Rc::new(return_env));
                            }

                            return Ok(current_env);
                        },
                        _ => return Err(ParserError::UnexpectedReturn(pos, self.line)),
                    }
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::FullStop(op) => {
                    let prev_operand: Option<Rc<AstNode>> = if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.pop()
                    } else {
                        None
                    };
                    if let Some(prev_operand) = prev_operand {
                        let env_rc: Rc<AstNode> = Rc::new(current_env.clone());
                        let node: AstNode = self.parse_operator(Some(env_rc), &Operators::Other(op.clone()), &prev_operand)?;
                        if let AstNode::Environment { ref mut bindings, .. } = current_env {
                            bindings.push(Rc::new(node));
                        }
                    } else {
                        return Err(ParserError::BinaryOpWithNoLHS(pos, self.line));
                    }
                },
                Token::Operator(op) => {
                    let prev_operand: Option<Rc<AstNode>> = if let AstNode::Environment { ref mut bindings, .. } = current_env {
                        bindings.pop()
                    } else {
                        None
                    };
                    if let Some(prev_operand) = prev_operand {
                        let env_rc: Rc<AstNode> = Rc::new(current_env.clone());
                        let node: AstNode = self.parse_operator(Some(env_rc), op, &prev_operand)?;
                        if let AstNode::Environment { ref mut bindings, .. } = current_env {
                            bindings.push(Rc::new(node));
                        }
                    } else {
                        return Err(ParserError::BinaryOpWithNoLHS(pos, self.line));
                    }
                },
                Token::LineTerminator(_) => {
                    match context {
                        ParseContext::Operation => {
                            // Return the right-hand side of the operation
                            if let AstNode::Environment { ref bindings, .. } = current_env {
                                if bindings.len() == 1 {
                                    return Ok((*bindings[0]).clone());
                                }
                            }
                        },
                        ParseContext::Normal => {
                            // Return the current environment if it is named
                            if name.is_some() {
                                return Ok(current_env.clone());
                            }
                        },
                        ParseContext::Function => {
                            continue;
                        },
                        ParseContext::FunctionReturn => {
                            return Ok(current_env.clone());
                        }
                    }
                },
                Token::EOF => {
                    match context {
                        ParseContext::FunctionReturn => {
                            // Return statements can finish on EOF
                            return Ok(current_env);
                        },
                        ParseContext::Normal if parent.is_none() => {
                            // Global env can finish on EOF
                            return Ok(current_env);
                        },
                        ParseContext::Normal => {
                            // Non-global env cannot finish on EOF
                            return Err(ParserError::UnexpectedEOF(pos, self.line));
                        },
                        ParseContext::Function => {
                            // Functions cannot finish without return statements
                            return Err(ParserError::MissingReturnStatement(pos, self.line, "".into()))
                        },
                        ParseContext::Operation => {
                            // Operations cannot finish on EOF
                            return Err(ParserError::UnexpectedEOF(pos, self.line));
                        }
                    }
                },
            }
        }

        // Check if EOF token was consumed by one of the valid contexts
        match context {
            ParseContext::Normal if parent.is_none() => {
                return Ok(current_env);
            },
            ParseContext::FunctionReturn => {
                return Ok(current_env);
            },
            _ => Err(ParserError::UnclosedEnvironment(self.line))
        }
    }

    /// Returns an [`AstNode::Let`] representing an assignment operation.
    /// 
    /// # Arguments
    /// * `parent_env`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// 
    /// # Errors
    /// * Any error bubbled up from [`parse_function_declaration`](Parser::parse_function_declaration) or [`construct_let_statement`](Parser::construct_let_statement).
    /// * [`ParserError::MissingLetIdentifier`]: The "let" keyword was not followed by a valid identifier or the "fun" keyword.
    /// * [`ParserError::ParserLogicError`]: The call to `parse_assignment` was triggered from the final token in the token vector.
    fn parse_assignment(&mut self, parent_env: Option<Rc<AstNode>>) -> Result<AstNode, ParserError> {
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Keyword(Keywords::FUN) => {
                    return Ok(self.parse_function_declaration(&parent_env)?);
                }, 
                Token::Identifier(id) => {
                    return Ok(self.construct_let_statement(&parent_env, id, ParseContext::Normal)?);
                },
                _ => return Err(ParserError::MissingLetIdentifier(pos, self.line)), 
            }
        }
        Err(ParserError::ParserLogicError(self.current, self.line))
    }

    /// Returns an [`AstNode::Function`] representing a function declaration.
    /// 
    /// # Arguments
    /// * `parent_env`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_function_clause`](Parser::parse_function_clause), [`parse_inherit_clause`](Parser::parse_inherit_clause), and [`parse_environment`](Parser::parse_environment).
    /// * [`ParserError::MissingFunctionName`]: The "fun" keyword was not followed by a valid identifier.
    /// * [`ParserError::MissingFunctionArgs`]: The function identifier was not immediately (no whitespace allowed) followed by the left bracket symbol for function arguments.
    /// * [`ParserError::InvalidTokenInFnSignature`]: The function arguments were not followed by either an "inherit" clause or the assignment operator.
    /// * [`ParserError::MissingReturnStatement`]: The return statement parsing failed without error, suggesting that there was no return statement in the source code.
    /// * [`ParserError::MissingAssignmentOp`]: The function arguments, with an optional "inherit" clause, were not followed by an assignment operator.
    /// * [`ParserError::MissingFunctionBody`]: The parsing of the function body immediately following the assignment operator failed without error, suggesting that there is no function body.
    fn parse_function_declaration(&mut self, parent_env: &Option<Rc<AstNode>>) -> Result<AstNode, ParserError> {
        // Temporary variables to store required components (ordered by Envlang syntax for ease of reading)
        let mut fn_name: Option<Rc<str>> = None;
        let mut fn_args: Option<AstNode> = None;
        let mut inheritance: Option<Rc<AstNode>> = None;
        let mut fn_body: Option<AstNode> = None;
        let mut fn_return: Option<AstNode> = None;

        // Step 1: Parse function name
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Identifier(id) => {
                    fn_name = Some(id.clone());
                    break;
                },
                _ => return Err(ParserError::MissingFunctionName(pos, self.line, token.to_string())),
            }
        }

        // Step 2: Parse function arguments (no allowed whitespace between name and arguments)
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::LeftBracket(_) => {
                    fn_args = Some(self.parse_function_clause()?);
                    break;
                },
                _ => return Err(ParserError::MissingFunctionArgs(pos, self.line)),
            }
        }

        // Step 3: Parse optional inheritance clause
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Keyword(Keywords::INHERIT) => {
                    inheritance = Some(Rc::new(self.parse_inherit_clause()?));
                    break;
                },
                Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)) => {
                    // Backtracking to use operator in next step
                    self.current -= 1;
                    break;
                },
                _ => return Err(ParserError::InvalidTokenInFnSignature(pos, self.line, token.to_string())),
            }
        }

        // Step 4: Parse assignment operator, function body, and return statement
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)) => {
                    let body = self.parse_environment(
                        parent_env.clone(),
                        fn_name.clone(),
                        ParseContext::Function
                    )?;
                    
                    if let AstNode::Environment { bindings, .. } = &body {
                        if let Some(last) = bindings.last() {
                            fn_body = Some(AstNode::Environment {
                                name: fn_name.clone(),
                                bindings: bindings[..bindings.len()-1].to_vec(),
                                parent: parent_env.clone(),
                            });
                            // I apologize for the following disgusting pointer indirection...
                            if let AstNode::Environment { bindings: return_bindings, .. } = &**last {
                                fn_return = Some(AstNode::Environment {
                                    name: None,
                                    bindings: return_bindings.clone(),
                                    parent: parent_env.clone(),
                                });
                            }
                            break;
                        }
                    }
                    return Err(ParserError::MissingReturnStatement(pos, self.line, "".into()));
                },
                _ => return Err(ParserError::MissingAssignmentOp(pos, self.line)),
            }
        }

        // Validate that required components were parsed
        let fn_name = fn_name.ok_or_else(|| ParserError::MissingFunctionName(self.current, self.line, "".into()))?;
        let fn_args = fn_args.ok_or_else(|| ParserError::MissingFunctionArgs(self.current, self.line))?;
        let fn_body = fn_body.ok_or_else(|| ParserError::MissingFunctionBody(self.current, self.line))?;
        let fn_return = fn_return.ok_or_else(|| ParserError::MissingReturnStatement(self.current, self.line, "".into()))?;

        // Construct the complete function node
        Ok(AstNode::Let {
            name: fn_name,
            value: Some(Rc::new(AstNode::Function {
                params: Rc::new(fn_args),
                body: Rc::new(fn_body),
                r#return: Rc::new(fn_return),
            })),
            inherit: inheritance,
        })
    }

    /// Returns an [`AstNode::FunctionArgs`] representing the arguments of a function.
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_identifier`](Parser::parse_identifier).
    /// * [`ParserError::DoubleFunArgBracket`]: Two (or more) left brackets in the function argument clause.
    /// * [`ParserError::UnclosedArgumentClause`]: EOF token met before finishing the argument clause.
    /// * [`ParserError::InvalidFunArgToken`]: Any other token than identifiers, commas, or the EOF token met before finishing the argument clause.
    /// * [`ParserError::ParserLogicError`]: Parser somehow finished the token stream without errors (catch-all for seemingly impossible scenarios).
    fn parse_function_clause(&mut self) -> Result<AstNode, ParserError> {
        let mut result = AstNode::FunctionArgs(Vec::new());
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::LeftBracket(_) => {
                    // If the element vector is non-empty, this represents a syntax error
                    if let Some(names) = result.get_params() {
                        if !names.is_empty() {
                            return Err(ParserError::DoubleFunArgBracket(pos, self.line, token.to_string()));
                        }
                    }
                    continue;
                },
                Token::RightBracket(_) => {
                    // Finish parsing identifiers and return
                    return Ok(result);
                },
                Token::Comma => {
                    continue;
                },
                Token::Identifier(id) => {
                    // Add identifier pointer to vector
                    result.set_field::<AstNode>(|v| {
                        let AstNode::FunctionArgs(args) = v else {
                            unreachable!("Safety: Will always be AstNode::FunctionArgs");
                        };
                        let node = self.parse_identifier(&id)?;
                        args.push(Rc::new(node));
                        Ok(())
                    })?;
                    continue;
                },
                Token::EOF => {
                    // ERROR: Unclosed argument clause
                    return Err(ParserError::UnclosedArgumentClause(self.line));
                }
                _ => {
                    // ERROR: Not a valid symbol in a function clause (could be unclosed argument clause!)
                    return Err(ParserError::InvalidFunArgToken(pos, self.line, token.to_string()));
                },
            }
        }
        return Err(ParserError::ParserLogicError(self.current, self.line));
    }

    /// Returns an [`AstNode::Let`] representing the assignment.
    /// 
    /// # Arguments
    /// * `parent_env`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// * `id`: A reference-counted pointer to the name of the assignment.
    /// * `context`: A [`ParseContext`] representing the context within which the assignment is done.
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_inherit_clause`](Parser::parse_inherit_clause), and [`flatten_environment`].
    /// * [`ParserError::InvalidAssignmentOp`]: Any other operator than the assignment operator encountered after either an identifier or an inheritance clause.
    /// * [`ParserError::MissingAssignmentOp`]: No assignment operator was found.
    /// * [`ParserError::ParserLogicError`]: Parser somehow finished the token stream without errors (catch-all for seemingly impossible scenarios).
    fn construct_let_statement(&mut self, parent_env: &Option<Rc<AstNode>>, id: &Rc<str>, context: ParseContext) -> Result<AstNode, ParserError> {
        let mut result = AstNode::Let {
            name: id.clone(),
            value: None,
            inherit: None
        };

        // TODO: A function call context should not contain an inheritance clause (though it is a soft error). We can check this from the "inherit" field of result.
        // TODO: The function call will have argument bracket notation in-between the identifier (outside this context) and the assignment operator (in the match arm here)
    
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::Keyword(Keywords::INHERIT) => {
                    let inheritance = self.parse_inherit_clause()?; 

                    // Modify the mutable Let object from above to contain the inheritance clause on the inherit element
                    result.set_field::<AstNode>(|v| {
                        if let AstNode::Let{ inherit, .. } = v {
                            *inherit = Some(Rc::new(inheritance));
                        }
                        Ok(())
                    }).expect("Safety: Will always be AstNode::Let");
                    continue;
                },
                Token::Operator(op) => {
                    if *op == Operators::Other(OtherOperators::ASSIGNMENT) {
                        let expr: AstNode = self.parse_environment(parent_env.clone(), Some(id.clone()), context)?;
                        
                        let let_env: Rc<AstNode>;
                        if expr.is_single_element_env() {
                            let_env = flatten_environment(&expr, pos, self.line, &token)?;
                        } else {
                            let_env = Rc::new(expr);
                        }

                        result.set_field::<AstNode>(|v| {
                            if let AstNode::Let{ value, .. } = v {
                                *value = Some(let_env);
                            }
                            Ok(())
                        }).expect("Safety: Will always be AstNode::Let");

                        return Ok(result);
                    } else {
                        return Err(ParserError::InvalidAssignmentOp(pos, self.line, token.to_string()));
                    }
                },
                _ => return Err(ParserError::MissingAssignmentOp(pos, self.line)),
            }
        }
        Err(ParserError::ParserLogicError(self.current, self.line))
    }

    /// Returns an [`AstNode::Inherit`] representing the inheritance clause.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Errors
    /// * [`ParserError::DoubleInheritanceParen`]: Two (or more) left parentheses encountered in the inheritance clause.
    /// * [`ParserError::WildcardAndElements`]: The inheritance clause contained both a wildcard and one (or more) inheritance element(s).
    /// * [`ParserError::InvalidInheritanceToken`]: The inheritance clause contained some other token than parentheses, commas, the wildcard operator, or identifiers.
    fn parse_inherit_clause(&mut self) -> Result<AstNode, ParserError> {
        let mut inheritance_arg = AstNode::Inherit { names: Some(Vec::new()) };
        while let Some((pos, token)) = self.advance() {
            match token.borrow() {
                Token::Whitespace(ws) => self.parse_whitespace(ws),
                Token::LeftParen(_) => {
                    // If the element vector is non-empty, this represents a syntax error
                    if let Some(names) = inheritance_arg.get_inherited_names() {
                        if !names.is_empty() {
                            return Err(ParserError::DoubleInheritanceParen(pos, self.line, token.to_string()));
                        }
                    }
                    continue;
                },
                Token::RightParen(_) => {
                    return Ok(inheritance_arg);
                },
                Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)) => {
                    if let Some(names) = inheritance_arg.get_inherited_names() {
                        if !names.is_empty() {
                            return Err(ParserError::WildcardAndElements(pos, self.line, token.to_string()));
                        }
                    }
                    if let AstNode::Inherit { ref mut names } = inheritance_arg {
                        *names = None;
                    }
                }
                Token::Identifier(id) => {
                    // None represents a previous wildcard, which cannot be combined with identifiers
                    match inheritance_arg.get_inherited_names() {
                        Some(_) => (),
                        None => return Err(ParserError::WildcardAndElements(pos, self.line, token.to_string())),
                    }

                    inheritance_arg.push_inherited_name(id.clone())
                        .expect("Safety: Will always be AstNode::Inherit");
                    continue;
                },
                Token::Comma => {
                    continue;
                },
                _ => {
                    return Err(ParserError::InvalidInheritanceToken(pos, self.line, token.to_string()))
                }
            }
        }
        return Ok(inheritance_arg);
    }
    
    /// Returns an [`AstNode::String`] containing the string literal.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Arguments
    /// * `string`: A reference-counted pointer to the string literal (as `str`) to be converted.
    /// 
    /// # Panics
    /// * Failed cloning of string pointer
    /// * Failed wrapping of string pointer clone into `AstNode::String`
    fn parse_string(&mut self, string: &Rc<str>) -> Result<AstNode, ParserError> { Ok(AstNode::String(string.clone())) }

    /// Returns an [`AstNode::Identifier`] containing the identifier.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Arguments
    /// * `id`: A reference-counted pointer to the identifier (as `str`) to be converted.
    /// 
    /// # Panics
    /// * Failed cloning of identifier pointer.
    /// * Failed wrapping of identifier pointer clone into `AstNode::Identifier`.
    fn parse_identifier(&mut self, id: &Rc<str>) -> Result<AstNode, ParserError> { Ok(AstNode::Identifier(id.clone())) }

    /// Increments the line counter if the whitespace is a new-line character.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Arguments
    /// * `ws`: A reference-counted pointer to the whitespace (as `str`) to be converted.
    /// 
    /// The method does not panic or return errors.
    fn parse_whitespace(&mut self, ws: &Rc<str>) { match ws.borrow() { "\r\n" | "\n" => self.line += 1, _ => () }}

    /// Returns an `[AstNode::BinaryOp`] representing the binary operation.
    /// 
    /// # Arguments
    /// * `parent_env`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// * `op`: A reference to the operator enum variant.
    /// * `prev`: A reference-counted pointer to the previous (left-hand-side) element (as [`AstNode`]).
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_accessor_op`](Parser::parse_accessor_op), and [`parse_generic_op`](Parser::parse_accessor_op).
    fn parse_operator(&mut self, parent_env: Option<Rc<AstNode>>, op: &Operators, prev: &Rc<AstNode>) -> Result<AstNode, ParserError> {
        match op {
            Operators::Other(OtherOperators::ACCESSOR) => {
                return Ok(self.parse_accessor_op(op, prev)?)
            },
            _ => {
                return Ok(self.parse_generic_op(parent_env, op, prev)?)
            },
        }
    }

    /// Returns an [`AstNode::BinaryOp`] representing a generic, non-accession binary operation.
    /// 
    /// # Arguments
    /// * `parent_env`: An `Option`al reference-counted pointer to the parent environment, or `None` for the global environment.
    /// * `op`: A reference to the operator enum variant.
    /// * `prev`: A reference-counted pointer to the previous (left-hand-side) element (as [`AstNode`]).
    /// 
    /// # Errors
    /// * Any errors bubbled up from [`parse_environment`](Parser::parse_environment).
    /// * [`ParserError::UnexpectedEOF`]: The token stream was unexpectedly empty.
    fn parse_generic_op(&mut self, parent_env: Option<Rc<AstNode>>, op: &Operators, prev: &Rc<AstNode>) -> Result<AstNode, ParserError> {
        if let Some(_) = self.peek() {
            let next_node: AstNode = self.parse_environment(parent_env, None, ParseContext::Operation)?;
        
            // Flatten single-item environments into a single node
            if let Some(bindings) = next_node.get_bindings() {
                if bindings.len() == 1 {
                    return Ok(AstNode::BinaryOp {
                        left: prev.clone(),
                        operator: op.clone(),
                        right: bindings[0].clone()
                    });
                }
            }
            // Return multi-item rhs environments as is
            return Ok(AstNode::BinaryOp {
                left: prev.clone(),
                operator: op.clone(),
                right: Rc::new(next_node)
            });
        } else {
            return Err(ParserError::UnexpectedEOF(self.current, self.line));
        }
    }
    
    /// Returns an [`AstNode::BinaryOp`] representing the accession operation.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Arguments
    /// * `op`: A reference to the operator enum variant.
    /// * `prev`: A reference-counted pointer to the previous (left-hand-side) element (as [`AstNode`]).
    /// 
    /// # Errors
    /// * [`ParserError::ParserLogicError`]: The operator given to the method was not an accessor operator (indicating an implementation error in Envlang).
    /// * [`ParserError::InvalidAccessionTarget`]: The accessor operator was used on any non-identifier left-hand-side operand.
    /// * [`ParserError::UnexpectedEOF`]: The token stream unexpectedly ended.
    /// * [`ParserError::InvalidAccessionSource`]: The accessor operator was used on a right-hand-side operand being something else than an identifier or environment.
    fn parse_accessor_op(&mut self, op: &Operators, prev: &Rc<AstNode>) -> Result<AstNode, ParserError> {
        // Exit early if the operator is not an accessor
        match op {
            Operators::Other(OtherOperators::ACCESSOR) => (),
            _ => return Err(ParserError::ParserLogicError(self.current, self.line))
        }

        match prev.borrow() {
            AstNode::Identifier(_) | AstNode::Environment{ .. } => {
                if let Some((pos, token)) = self.advance() {
                    match token.borrow() {
                        Token::Identifier(id) => {
                            return Ok(AstNode::BinaryOp {
                                left: prev.clone(),
                                operator: op.clone(),
                                right: Rc::new(AstNode::Identifier(id.clone()))
                            });
                        },
                        _ => return Err(ParserError::InvalidAccessionTarget(pos, self.line, token.to_string()))
                    }
                }
                return Err(ParserError::UnexpectedEOF(self.current, self.line));
            },
            _ => return Err(ParserError::InvalidAccessionSource(self.current, self.line, prev.to_string())),
        }
    }
    
    /// Returns an [`AstNode::Integer`] or [`AstNode::Float`] representing the number type and data.
    /// 
    /// This is a bottom-level submethod and does not call other submethods.
    /// 
    /// # Arguments
    /// * `start_pos`: An unsigned integer representing the starting index of the number in the token stream.
    /// * `start_token`: A reference to the starting token (as `Token`).
    /// 
    /// # Errors
    /// * [`ParserError::NotANumber`]: The declared number does not start with a number or full-stop, or a parsed number does not map into `isize` or `f64`.
    /// * [`ParserError::MalformedNumber`]: The declared number contains two (or more) full-stops.
    fn parse_number(&mut self, start_pos: usize, start_token: &Token) -> Result<AstNode, ParserError> {
        let mut numstr: String = String::new();
        // Valid numbers start with a number or a full stop (if float)
        match start_token {
            Token::Number(num) => numstr.push_str(num),
            Token::FullStop(_) => numstr.push_str("0."),
            _ => return Err(ParserError::NotANumber(start_pos, self.line, numstr)),
        }
        
        while let Some(token) = self.peek() {
            match token {
                Token::Number(num) => {
                    numstr.push_str(num);
                    self.next();
                },
                Token::FullStop(_) => {
                    if numstr.contains(".") { // A float can only have one decimal point
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

    /// Debugging function to print the next token in the token queue
    #[allow(dead_code)]
    fn debug_next_token(&self) {
        println!("Next token at index: {}, token: {:?}", self.current, self.peek());
    }

    /// Debugging function to print a pos-token tuple
    #[allow(dead_code)]
    fn debug_token_tuple(&self, pos: usize, token: &Token) {
        println!("Grabbed token at index: {}, token: {:?}", pos, token);
    }
}

/// Takes a single-element environment and returns its binding as a pointer.
/// 
/// # PLANNED CHANGES
/// This associated function will be made a method in the future, since it utilizes parser information for error handling.
/// 
/// # Arguments
/// * `expr`: A reference to the `AstNode` which is to be flattened.
/// * `pos`: The position of the parser.
/// * `line`: The line number of the parser.
/// * `token`: A reference to the `Token` that the parser is currently at.
/// 
/// # Guarantees
/// The caller must make sure that the [`AstNode::Environment`] has exactly zero or one binding.
/// 
/// If the `AstNode::Environment` has more than one binding, the function only returns the first binding.
/// 
/// # Errors
/// * [`ParserError::EmptyEnv`]: The supplied environment has zero bindings.
/// * [`ParserError::NotAnEnvironment`]: The supplied element is not an environment.
fn flatten_environment(expr: &AstNode, pos: usize, line: usize, token: &Token) -> Result<Rc<AstNode>, ParserError> {
    match expr {
        AstNode::Environment{ bindings, .. } => {
            if bindings.len() == 0 {
                return Err(ParserError::EmptyEnv(pos, line, token.to_string()));
            }
            return Ok(bindings[0].clone());
        },
        _ => return Err(ParserError::NotAnEnvironment(pos, line, token.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::{ReservedSymbols, Operators, ArithmeticOperators};

    // Basic cases
    #[test]
    fn int() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Integer(5))], parent: None });
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
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Float(5.0))], parent: None });
    }

    #[test]
    fn string_literal() {
        let tokens = vec![
            Token::StringLiteral("Hello, world!".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::String("Hello, world!".into()))], parent: None });
    }

    #[test]
    fn identifier() {
        let tokens = vec![
            Token::Identifier("x".into()),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment { name: None, bindings: vec![Rc::new(AstNode::Identifier("x".into()))], parent: None });
    }

    #[test]
    fn assignment() {
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
                value: Some(Rc::new(AstNode::Integer(5))),
                inherit: None,
            })],
            parent: None
        });
    }
    
    #[test]
    fn operation() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Number("3".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Integer(5)),
                operator: Operators::Arithmetic(ArithmeticOperators::ADD),
                right: Rc::new(AstNode::Integer(3))
            })],
            parent: None
        });
    }

    #[test]
    fn accession() {
        let tokens = vec![
            Token::Identifier("x".into()),
            Token::FullStop(OtherOperators::ACCESSOR),
            Token::Identifier("y".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Identifier("x".into())),
                operator: Operators::Other(OtherOperators::ACCESSOR),
                right: Rc::new(AstNode::Identifier("y".into())),
            })],
            parent: None
        });
    }

    // Complex cases
    #[test]
    fn assignment_with_identifier() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Identifier("y".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "x".into(),
                value: Some(Rc::new(AstNode::Identifier("y".into()))),
                inherit: None,
            })],
            parent: None
        });
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
        
        if let AstNode::Environment { bindings, .. } = ast {
            assert_eq!(bindings.len(), 2);
            // First binding should be a local environment containing 1
            if let AstNode::Environment { bindings: sub_bindings, .. } = &*bindings[0] {
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
    fn environment_with_assignment() {
        let tokens = vec![
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let AstNode::Environment { bindings, .. } = ast {
            assert_eq!(bindings.len(), 1);
            // First binding should be a local environment containing the assignment
            if let AstNode::Environment { bindings: sub_bindings, .. } = &*bindings[0] {
                assert_eq!(sub_bindings.len(), 1);
                assert_eq!(sub_bindings[0], Rc::new(AstNode::Let {
                    name: "x".into(),
                    value: Some(Rc::new(AstNode::Integer(5))),
                    inherit: None,
                }));
            }
        } else {
            panic!("Expected Environment node");
        }
    }

    #[test]
    fn nested_operation() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Number("3".into()),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Integer(5)),
                operator: Operators::Arithmetic(ArithmeticOperators::ADD),
                right: Rc::new(AstNode::Integer(3))
            })], parent: None
        });
    }

    #[test]
    fn operation_with_extra_whitespace() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::Whitespace(" ".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Whitespace("\n".into()),
            Token::Number("3".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Integer(5)),
                operator: Operators::Arithmetic(ArithmeticOperators::ADD),
                right: Rc::new(AstNode::Integer(3))
            })], parent: None
        });
    }

    #[test]
    fn assignment_with_specified_inheritance() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Identifier("b".into()),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
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
                value: Some(Rc::new(AstNode::Integer(5))),
                inherit: Some(Rc::new(AstNode::Inherit {
                    names: Some(vec![
                        "a".into(),
                        "b".into(),
                    ])
                })),
            })],
            parent: None
        });
    }

    #[test]
    fn assignment_with_wildcard_inheritance() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
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
                value: Some(Rc::new(AstNode::Integer(5))),
                inherit: Some(Rc::new(AstNode::Inherit {
                    names: None
                })),
            })],
            parent: None
        });
    }

    // Function tests
    #[test]
    fn minimal_function_assignment() {
        // Named function that returns a single element without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment{
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![Rc::new(AstNode::Integer(5))],
                        parent: Some(global_env.clone())
                    }),
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_return_env() {
        // Function that returns an empty explicit environment
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_arguments() {
        // Function that takes two arguments and returns a single element without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![
                        Rc::new(AstNode::Identifier("x".into())),
                        Rc::new(AstNode::Identifier("y".into()))
                    ])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![Rc::new(AstNode::Integer(5))],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_inheritance() {
        // Function that inherits two elements and returns a single element without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![Rc::new(AstNode::Integer(5))],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: Some(Rc::new(AstNode::Inherit {
                    names: Some(vec![
                        "x".into(),
                        "y".into()
                    ])
                })),
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_wildcard_inheritance() {
        // Function that inherits everything and returns a single element without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![Rc::new(AstNode::Integer(5))],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: Some(Rc::new(AstNode::Inherit {
                    names: None
                })),
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_extensive_body() {
        // Function whose body assigns two variables and returns an operation without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("1".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::Keyword(Keywords::LET),
            Token::Identifier("y".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("2".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::Keyword(Keywords::RETURN),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Identifier("y".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment{
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![
                            Rc::new(AstNode::Let {
                                name: "x".into(),
                                value: Some(Rc::new(AstNode::Integer(1))),
                                inherit: None,
                            }),
                            Rc::new(AstNode::Let {
                                name: "y".into(),
                                value: Some(Rc::new(AstNode::Integer(2))),
                                inherit: None,
                            })
                        ],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![
                            Rc::new(AstNode::BinaryOp {
                                left: Rc::new(AstNode::Identifier("x".into())),
                                operator: Operators::Arithmetic(ArithmeticOperators::ADD),
                                right: Rc::new(AstNode::Identifier("y".into()))
                            })
                        ],
                        parent: Some(global_env.clone())
                    }),
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_body_and_args() {
        // Function that takes two arguments and returns an operation without explicit environment encapsulation
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Keyword(Keywords::RETURN),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Identifier("y".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment{
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![
                        Rc::new(AstNode::Identifier("x".into())),
                        Rc::new(AstNode::Identifier("y".into()))
                    ])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![
                            Rc::new(AstNode::BinaryOp {
                                left: Rc::new(AstNode::Identifier("x".into())),
                                operator: Operators::Arithmetic(ArithmeticOperators::ADD),
                                right: Rc::new(AstNode::Identifier("y".into()))
                            })
                        ],
                        parent: Some(global_env.clone())
                    }),
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_single_element_return_env() {
        // Function that returns a single element with environment encapsulation.
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Number("5".into()),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![Rc::new(AstNode::Integer(5))],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    #[test]
    fn function_decl_with_large_return_env() {
        // Function that returns an encapsulated multi-element environment
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Keyword(Keywords::FUN),
            Token::Identifier("foo".into()),
            Token::LeftBracket(ReservedSymbols::FUNARGOPEN),
            Token::RightBracket(ReservedSymbols::FUNARGCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace(ReservedSymbols::ENVOPEN),
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::Keyword(Keywords::LET),
            Token::Identifier("y".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("3".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::Keyword(Keywords::LET),
            Token::Identifier("z".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("1".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::RightBrace(ReservedSymbols::ENVCLOSE),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let global_env = Rc::new(AstNode::Environment {
            name: None,
            bindings: vec![],
            parent: None
        });

        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::Let {
                name: "foo".into(),
                value: Some(Rc::new(AstNode::Function {
                    params: Rc::new(AstNode::FunctionArgs(vec![])),
                    body: Rc::new(AstNode::Environment {
                        name: Some("foo".into()),
                        bindings: vec![],
                        parent: Some(global_env.clone())
                    }),
                    r#return: Rc::new(AstNode::Environment {
                        name: None,
                        bindings: vec![
                            Rc::new(AstNode::Let {
                                name: "x".into(),
                                value: Some(Rc::new(AstNode::Integer(5))),
                                inherit: None,
                            }),
                            Rc::new(AstNode::Let {
                                name: "y".into(),
                                value: Some(Rc::new(AstNode::Integer(3))),
                                inherit: None,
                            }),
                            Rc::new(AstNode::Let {
                                name: "z".into(),
                                value: Some(Rc::new(AstNode::Integer(1))),
                                inherit: None,
                            })
                        ],
                        parent: Some(global_env.clone())
                    })
                })),
                inherit: None,
            })],
            parent: None
        });
    }

    // Error cases
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
        assert_eq!(ast.unwrap_err(), ParserError::BinaryOpWithNoLHS(0, 1))
    }

    #[test]
    fn cannot_inherit_specified_before_wildcard() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::WildcardAndElements(6, 1, "*".into()))
    }

    #[test]
    fn cannot_inherit_wildcard_before_specified() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen(ReservedSymbols::INHERITOPEN),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::Comma,
            Token::Identifier("a".into()),
            Token::RightParen(ReservedSymbols::INHERITCLOSE),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator(ReservedSymbols::TERMINATOR),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::WildcardAndElements(6, 1, "a".into()))
    }
}