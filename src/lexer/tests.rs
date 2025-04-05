#[cfg(test)]
mod tests {
    use crate::symbols::{ArithmeticOperators, ComparisonOperators, OtherOperators, Operators, Booleans, Keywords};
    use crate::lexer::{Lexer, LexerError, Token};
    use std::rc::Rc;

    // Error condition tests
    #[test]
    fn error_empty_string() {
        let input = vec!["".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens,
            Err(LexerError::InvalidToken(pos, s)) if pos == 0 && s.is_empty()
        ));
    }

    #[test]
    fn error_unterminated_string() {
        let input = vec!["\"".to_string(), "hello".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens, 
            Err(LexerError::UnterminatedString(pos, s)) if pos == 0 && s == "hello"
        ));
    }

    #[test]
    fn error_special_character() {
        let input = vec!["@".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(matches!(tokens, 
            Err(LexerError::UnrecognizedInput(pos, s)) if pos == 0 && s == "@"
        ));
    }

    // Tests for correct behaviour
    #[test]
    fn matches_left_brace() {
        let input = vec!["{".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LeftBrace, Token::EOF]);
    }

    #[test]
    fn matches_right_brace() {
        let input = vec!["}".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::RightBrace, Token::EOF]);
    }

    #[test]
    fn matches_left_parenthesis() {
        let input = vec!["(".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LeftParen, Token::EOF]);
    }

    #[test]
    fn matches_right_parenthesis() {
        let input = vec![")".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::RightParen, Token::EOF]);
    }

    #[test]
    fn matches_comma() {
        let input = vec![",".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Comma, Token::EOF]);
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
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("asd")), Token::EOF]);
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
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("asd")), Token::EOF]);
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
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("'asd'")), Token::EOF])
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
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("\"asd\"")), Token::EOF])
    }

    #[test]
    fn matches_add_operator() {
        let input = vec!["+".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::ADD)), Token::EOF]);
    }

    #[test]
    fn matches_subtract_operator() {
        let input = vec!["-".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::SUBTRACT)), Token::EOF]);
    }

    #[test]
    fn matches_multiply_operator() {
        let input = vec!["*".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::MULTIPLY)), Token::EOF]);
    }

    #[test]
    fn matches_divide_operator() {
        let input = vec!["/".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::DIVIDE)), Token::EOF]);
    }

    #[test]
    fn matches_modulus_operator() {
        let input = vec!["%".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::MODULUS)), Token::EOF]);
    }

    #[test]
    fn matches_exponentiation_operator() {
        let input = vec!["^".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Arithmetic(ArithmeticOperators::EXPONENTIATION)), Token::EOF]);
    }

    #[test]
    fn matches_fullstop() {
        let input = vec![".".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Other(OtherOperators::ACCESSOR)), Token::EOF]);
    }

    #[test]
    fn matches_assignment_operator() {
        let input = vec!["=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Other(OtherOperators::ASSIGNMENT)), Token::EOF]);
    }

    #[test]
    fn matches_digits() {
        let input = vec!["12345".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(Rc::from("12345")), Token::EOF]);
    }

    #[test]
    fn matches_whitespace() {
        let input = vec!["\n".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\n")), Token::EOF]);
    }

    #[test]
    fn matches_identifier() {
        let input = vec!["abc".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("abc")), Token::EOF]);
    }

    #[test]
    fn matches_bool_true() {
        let input = vec!["true".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::TRUE), Token::EOF])
    }

    #[test]
    fn matches_bool_false() {
        let input = vec!["false".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Boolean(Booleans::FALSE), Token::EOF])
    }

    #[test]
    fn matches_keyword_let() {
        let input = vec!["let".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::LET), Token::EOF])
    }

    #[test]
    fn matches_keyword_inherit() {
        let input = vec!["inherit".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::INHERIT), Token::EOF])
    }

    #[test]
    fn matches_keyword_fun() {
        let input = vec!["fun".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::FUN), Token::EOF])
    }

    #[test]
    fn matches_keyword_return() {
        let input = vec!["return".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Keyword(Keywords::RETURN), Token::EOF])
    }

    #[test]
    fn matches_line_terminator() {
        let input = vec![";".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::LineTerminator, Token::EOF]);
    }

    #[test]
    fn matches_lt_operator() {
        let input = vec!["<".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::LT)), Token::EOF]);
    }

    #[test]
    fn matches_leq_operator() {
        let input = vec!["<".to_string(), "=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::LEQ)), Token::EOF]);
    }

    #[test]
    fn matches_gt_operator() {
        let input = vec![">".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::GT)), Token::EOF]);
    }

    #[test]
    fn matches_geq_operator() {
        let input = vec![">".to_string(), "=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::GEQ)), Token::EOF]);
    }

    #[test]
    fn matches_eq_operator() {
        let input = vec!["=".to_string(), "=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::EQ)), Token::EOF]);
    }

    #[test]
    fn matches_neq_operator() {
        let input = vec!["!".to_string(), "=".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operators::Comparison(ComparisonOperators::NEQ)), Token::EOF]);
    }

    // Complex token sequence tests
    #[test]
    fn handles_alphabetic_followed_by_number() {
        let input = vec!["a".to_string(), "123".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier(Rc::from("a123")),
            Token::EOF
        ]);
    }

    #[test]
    fn handles_complex_identifier() {
        let input = vec!["abc".to_string(), "-".to_string(), "123".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier(Rc::from("abc-123")),
            Token::EOF
        ]);
    }

    // Tests for edge cases
    #[test]
    fn handles_diacratic_identifier() {
        let input = vec!["üýö".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Identifier(Rc::from("üýö")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_strings() {
        let input = vec!["\"".to_string(), "😺".to_string(), "\"".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::StringLiteral(Rc::from("😺")), Token::EOF]);
    }
    
    #[test]
    fn emojis_are_not_identifiers() {
        let input = vec!["😺".to_string()];
        let tokens = Lexer::new(input).tokenize();
        assert!(tokens.is_err());
    }

    #[test]
    fn handles_windows_newline() {
        let input = vec!["\r\n".to_string()];
        let tokens = Lexer::new(input).tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Whitespace(Rc::from("\r\n")), Token::EOF]);
    }

    #[test]
    fn error_peek_beyond_input() {
        let input = vec!["a".to_string()];
        let lexer = Lexer::new(input);
        let result = lexer.peek_n(1);
        assert!(matches!(result, 
            Err(LexerError::IndexOutOfBounds(next, n, len)) 
            if next == 0 && n == 1 && len == 1
        ));
    }

    #[test]
    fn error_peek_with_broken_lexer() {
        let input = vec!["a".to_string()];
        let mut lexer = Lexer::new(input);
        lexer.current = 2;
        let result = lexer.peek_n(0);
        assert!(matches!(result, 
            Err(LexerError::BrokenLexer(pos, len)) if pos == 2 && len == 1
        ));
    }
}