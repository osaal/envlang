# Version 0

This is the Envlang alpha development version. Expect large API changes and sudden breaking.

## Version 0.5

This version entails a complete rewrite of reading in `.envl` files with the CLI, segmenting them into Unicode-compliant characters, lexing the characters into Tokens and parsing the Tokens into an Abstract Syntax Tree (AST).

Each step in the rewrite is done in a new `patch` version to ensure clarity of changes.

### Version 0.5.5

#### Major changes

- The parser now handles the `let` keyword (but the implementation is temporarily bugged because of a lack of handling line terminators)

#### Minor changes

- Added new error types to `ParserError`

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