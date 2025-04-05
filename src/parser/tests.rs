#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::parser::{Parser, AstNode, ParserError};
    use crate::symbols::{Keywords, Operators, ArithmeticOperators, ComparisonOperators, LogicalOperators, OtherOperators, Booleans};
    use std::rc::Rc;

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
            Token::Operator(Operators::Other(OtherOperators::ACCESSOR)),
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
            Token::LineTerminator,
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
            Token::LineTerminator,
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
    fn comparison_operation() {
        let tokens = vec![
            Token::Boolean(Booleans::TRUE),
            Token::Operator(Operators::Comparison(ComparisonOperators::NEQ)),
            Token::Boolean(Booleans::FALSE),
            Token::LineTerminator,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Boolean(true)),
                operator: Operators::Comparison(ComparisonOperators::NEQ),
                right: Rc::new(AstNode::Boolean(false)),
            })],
            parent: None,
        })
    }

    #[test]
    fn logical_operator() {
        let tokens = vec![
            Token::Boolean(Booleans::TRUE),
            Token::Operator(Operators::Logical(LogicalOperators::AND)),
            Token::Boolean(Booleans::FALSE),
            Token::LineTerminator,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![Rc::new(AstNode::BinaryOp {
                left: Rc::new(AstNode::Boolean(true)),
                operator: Operators::Logical(LogicalOperators::AND),
                right: Rc::new(AstNode::Boolean(false)),
            })],
            parent: None,
        })
    }

    #[test]
    fn accession() {
        let tokens = vec![
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ACCESSOR)),
            Token::Identifier("y".into()),
            Token::LineTerminator,
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
            Token::LineTerminator,
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
        Token::LeftBrace,
        Token::Number("1".into()),
        Token::RightBrace,
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
            Token::LeftBrace,
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBrace,
            Token::Number("3".into()),
            Token::RightBrace,
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
            Token::LineTerminator,
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
            Token::LeftParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Identifier("b".into()),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
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
            Token::LeftParen,
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace,
            Token::RightBrace,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Keyword(Keywords::INHERIT),
            Token::LeftParen,
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("1".into()),
            Token::LineTerminator,
            Token::Keyword(Keywords::LET),
            Token::Identifier("y".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("2".into()),
            Token::LineTerminator,
            Token::Keyword(Keywords::RETURN),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Identifier("y".into()),
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Keyword(Keywords::RETURN),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)),
            Token::Identifier("y".into()),
            Token::LineTerminator,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace,
            Token::Number("5".into()),
            Token::RightBrace,
            Token::LineTerminator,
            Token::RightBrace,
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
            Token::LeftBracket,
            Token::RightBracket,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::LeftBrace,
            Token::Keyword(Keywords::RETURN),
            Token::LeftBrace,
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::Keyword(Keywords::LET),
            Token::Identifier("y".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("3".into()),
            Token::LineTerminator,
            Token::Keyword(Keywords::LET),
            Token::Identifier("z".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("1".into()),
            Token::LineTerminator,
            Token::RightBrace,
            Token::LineTerminator,
            Token::RightBrace,
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

    // Function calls
    #[test]
    fn minimal_function_call() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Identifier("foo".into()),
            Token::LeftBracket,
            Token::RightBracket,
            Token::LineTerminator,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![
                Rc::new(AstNode::Let {
                    name: "x".into(),
                    value: Some(Rc::new(AstNode::FunctionCall {
                        id: Rc::new(AstNode::Identifier("foo".into())),
                        args: Rc::new(AstNode::FunctionArgs(
                            vec![]
                        ))
                    })),
                    inherit: None,
                })
            ],
            parent: None,
        });
    }

    #[test]
    fn function_call_with_one_parameter() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Identifier("foo".into()),
            Token::LeftBracket,
            Token::Identifier("y".into()),
            Token::RightBracket,
            Token::LineTerminator,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![
                Rc::new(AstNode::Let {
                    name: "x".into(),
                    value: Some(Rc::new(AstNode::FunctionCall {
                        id: Rc::new(AstNode::Identifier("foo".into())),
                        args: Rc::new(AstNode::FunctionArgs(
                            vec![
                                Rc::new(AstNode::Identifier("y".into()))
                            ]
                        ))
                    })),
                    inherit: None,
                })
            ],
            parent: None,
        });
    }

    #[test]
    fn function_call_with_multiple_parameters() {
        let tokens = vec![
            Token::Keyword(Keywords::LET),
            Token::Identifier("x".into()),
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Identifier("foo".into()),
            Token::LeftBracket,
            Token::Identifier("y".into()),
            Token::Comma,
            Token::Identifier("z".into()),
            Token::Comma,
            Token::Identifier("a".into()),
            Token::RightBracket,
            Token::LineTerminator,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AstNode::Environment {
            name: None,
            bindings: vec![
                Rc::new(AstNode::Let {
                    name: "x".into(),
                    value: Some(Rc::new(AstNode::FunctionCall {
                        id: Rc::new(AstNode::Identifier("foo".into())),
                        args: Rc::new(AstNode::FunctionArgs(
                            vec![
                                Rc::new(AstNode::Identifier("y".into())),
                                Rc::new(AstNode::Identifier("z".into())),
                                Rc::new(AstNode::Identifier("a".into())),
                            ]
                        ))
                    })),
                    inherit: None,
                })
            ],
            parent: None,
        });
    }

    // Error cases
    // TODO: Fix this test once the error handling is fixed to be more informative
    #[test]
    fn malformed_number() {
        let tokens = vec![
            Token::Number("5".into()),
            Token::Operator(Operators::Other(OtherOperators::ACCESSOR)),
            Token::Number("0".into()),
            Token::Operator(Operators::Other(OtherOperators::ACCESSOR)),
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
            Token::Operator(Operators::Other(OtherOperators::ACCESSOR)),
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
            Token::LeftParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
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
            Token::LeftParen,
            Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)),
            Token::Comma,
            Token::Identifier("a".into()),
            Token::RightParen,
            Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)),
            Token::Number("5".into()),
            Token::LineTerminator,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());
        assert_eq!(ast.unwrap_err(), ParserError::WildcardAndElements(6, 1, "a".into()))
    }
}