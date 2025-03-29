# Version 0

This is the Envlang alpha development version. Expect large API changes and sudden breaking.

## Version 0.5

This version entails a complete rewrite of reading in `.envl` files with the CLI, segmenting them into Unicode-compliant characters, lexing the characters into Tokens and parsing the Tokens into an Abstract Syntax Tree (AST).

Each step in the rewrite is done in a new `patch` version to ensure clarity of changes.

### Version 0.5.9

#### Major changes

- Lexer now recognises the `return` keyword as well as parameter declarations in function signatures (`[]`).
- The `AstNode::Function` variant now takes a `r#return` field (escaped to avoid collisions with Rust's reserved keywords) with an `Rc<AstNode>` to whatever is to be returned from the function.
- The `AstNode::Function` variant no longer takes the `inherit` field. Inherited elements are captured in the surrounding `Let` variant instead.
- Added the `AstNode::FunctionArgs` variant to encapsulate declared function arguments. These are then encapsulated in the `params` field of the `AstNode::Function` variant.
- The generic field setter `AstNode::set_field<T>` now expects a closure that returns `Result<(), ParserError>` in order to support bubbling up errors when setting fields.
- `Parser::parse_environment` now uses fully enumerated matching (instead of using the catch-all `_` at the end) to guarantee robustness in future expansions of parsing. All new `Token` variants have been added, either with logic or `todo!()` macros, depending on their current status.
- `Parser::parse_assignment` now recognises the `fun` keyword and calls the new method `Parser::parse_function_declaration`.
- All function parsing is handled by `parse_function_declaration`, with multiple error states if the syntax is misspecified.
- `Parser::construct_let_statement` now takes a `ParserContext` enum variant.
- The associated function `Parser::flatten_let_expression` has been reworked and refactored into `flatten_expression`. It now flattens any `AstNode::Environment` with one element.


#### Minor changes

- Added lexer test for `return` keyword.
- Added the enum variants `ParseContext::Function` and `ParseContext::FunctionReturn`.
- Added the enum variants `Token::LeftBracket` and `Token::RightBracket`.
- Added the public method `AstNode::is_single_element_env` for checking whether an AstNode is an Environment variant with only a single element inside itself.
- Added the public method `AstNoded::get_params` for returning the parameters of an `AstNode::FunctionArgs` variant.
- Extended the generic field setter `AstNode::set_field<T>` to work with the `AstNode::FunctionArgs` variant.
- Added many new error variants to `ParserError` related to function declaration parsing.
- Cleaned up safety documentation when using `.expect()` on method calls guaranteed to return `Ok(T)`.
- Added unit test for function declaration.

### Version 0.5.8

#### Major changes

- Lexer now handles parentheses and commas for parsing inheritance arguments
- The parser now parses inheritance arguments for `let` statements. Arguments are given in the form `let x inherit (a, b, c)` or with the wildcard operator `let x inherit (*)`.
- The `AstNode` variant `Let` now takes an `Option`al `value` in order to handle default construction
- Added the generic field setter `set_field<T>` for `AstNode`, for manipulating struct variants. It currently only supports changing fields for `Let` variants.
- The associated function `flatten_let_expression` now returns the raw bindings of the let expression as `Rc<AstNode>`. Consequently, it no longer accepts the `id` of the let statement.

#### Minor changes

- Added lexer tests for parentheses and commas
- Added getters for all fields in `AstNode` environments
- Added error types for inheritance
- Added tests for parsing inheritance arguments

### Version 0.5.7

#### Major changes

- The parser now parses accession operations of the type `x.y`. Accessions can be done on both identifiers and environments, as long as the accession target is an identifier.
- Removed `bindings` from the `Parser` struct since they became unnecessary
- Removed old implementation of `parse_accessor()` due to being superceded by `parse_operator()`
- The operator parser now calls either a generic operator parsing method or an accession parsing method, depending on the input `Token`
- Moved the parsing of full-stops/accessor operators to a later spot in the environment parser, to allow for full-stops-as-decimal-points to take precedence when in valid contexts

#### Minor changes

- Added error types for `ParserError`
- Corrected comma usage in `parse_environment`
- Removed the old test case for floats with leading decimals, as the parser currently does not support testing these outside of assignment contexts
- Corrected tests to match correct error types

### Version 0.5.6

#### Major changes

- The environment parser now takes a `ParseContext` enum. Currently, the enum represents either `Normal` or `Operation` context, to variably conduct parsing of other elements.
- The environment parser now constructs the `Environment` AST node before matching tokens, to ensure that its parentage can be relayed to sub-methods when necessary.
- The parser now handles binary operators. Note, that the parser does not check the validity of the operation, only the syntax itself.

#### Minor changes

- Added documentation to `construct_let_statement()`, `parse_operator()` and `parse_number()`
- Added tests for operator and environment parsing
- Fixed issue where number parsing would error with whitespace after the number, when it should just finish parsing and continue

### Version 0.5.5

#### Major changes

- The parser now handles the `let` keyword using the methods `parse_assignment` and `construct_let_statement` as well as the associated function `flatten_let_expression`
- The parser now handles bare identifiers (this might change into an error!)
- The lexer now parses line terminators as their own `LineTerminator` token enum variants
- The lexed tokens `LeftBrace`, `RightBrace`, and `FullStop` now wrap appropriate symbol enums for consistency with other tokens

#### Minor changes

- Added new error types to `ParserError`
- Added trait derivations for `ReservedSymbols` and the `ToString` trait implementation
- Updated the documentation for all symbol enums
- Updated lexer and parser implementations and tests to match new token variant type wrappings
- Added new test to the lexer to cover the line terminator addition

### Version 0.5.4

#### Major changes

- Split up operator enums into `ArithmeticOperators` and `OtherOperators`
- Operators are now sub-enumerated in `symbols::Operators` (either `Arithmetic(ArithmeticOperators)` or `Other(OtherOperators)`)
- Lexer now returns operator enums as opposed to strings. Consequently, `AstNode` now takes an `Operators` enum variant in `BinaryOp`
- `ParserError` now contains many more error types to match parsing error states
- The `Parser` now holds the parsed items in a `bindings` vector
- Refactored `parse_program` into `parse_environment`, extending its usage to all Environment objects
- The Parser now handles environments and ends-of-file correctly
- Added scaffolding for parsing binary operators and environment accession

#### Minor changes

- Added trait derivations for `AstNode`
- Corrected existing tests and added new tests for new operator enum pattern
- Added the `ToString` trait implementation for `AstNode`, `ArithmeticOperators` and `OtherOperators`
- Corrected existing tests and added new tests for `Parser`

### Version 0.5.3

#### Major changes

- The parser has been rewritten. Its structure matches that of the `Lexer`. Parsing is initiated by calling the `parse()` method, which interally calls `parse_program()` to walk the input `Token` vector and match the tokens to respective `parse_[item]` sub-methods.
- Implemented parsing for strings, whitespace, and numbers (integers and floats).
- Implemented skeleton of operator parsing
- Added the enum `AstNode`, which represents the parsed abstract syntax tree nodes in the parser.
- Reworked the error type `ParserError` to match the structure of `LexerError`

#### Minor changes

- Clarified and updated grammar in `grammar.ebnf`
- Added many tests for `Parser`
- Added trait derivations for `symbols::ArithmethicOperators` and `lexer::Token`

### Version 0.5.2

#### Major changes

- Envlang library code is now organized into sub-modules, where declarations and implementations are coupled together, but split apart from other declarations and implementations. For instance, the lexer now lives in `envlang::lexer` and publicly exposes the crates `lexer::error` and `lexer::token`.
- The Lexer now throws and returns errors. See `LexerError` for an enumeration of all error types.
- String-based Tokens now contain reference-counted pointers to `str`s as opposed to regular `String`s. This reduces memory overhead and speeds up accessing and copying markedly.

#### Minor changes

- Lexer methods have been fixed to accommodate the changes in Token data types. All methods now return `Rc<str>`s in places they used to return `String`s.
- The methods `Lexer.get_input_length` and `Lexer.peek_n` now both return a Result with detailed error information.
- `Lexer.tokenize` has seen major internal changes due to the change in Token types, and now returns a Result with potential error states
- Most of the inner workings of `Lexer.tokenize` have been refactored out into their own `tokenize_` methods for better error handling and separation of concerns. These methods take the starting string to be 'worked through' as well as the current position, in order to accurately tokenize different tokens as well as provide detailed error information.
- Added new tests for `LexerError` types
- Fixed old `Lexer` tests to conform to new Token data types
- The method `iterate` now returns a tuple of the current position of the lexer and the token at that position: `(usize, Rc<str>)`
- The unnecessary method `get_input_slice` has been removed.

### Version 0.5.1

#### Major changes

Envlang is now offered as a library crate `envlang`. All modules have been made public for further use. The CLI still lives in `main.rs` and has a specific compilation target which simply uses the library crate.

#### Minor changes

-  Added test cases to `io::read_file`
-  Switched to taking in string slices for `unicodesegmenters::segment_graphemes` to reduce unnecessary copying
-  Added test cases to `unicodesegmenters::segment_graphemes`
-  Fix spelling errors and incorrect doctests in documentation

### Version 0.5.0

-  Added CHANGELOG.md
-  Corrected documentation of `io::read_file`
-  Added test cases to `io::read_file`