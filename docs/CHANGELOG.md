# Version 0

This is the Envlang alpha development version. Expect large API changes and sudden breaking.

## Version 0.5

This version entails a complete rewrite of reading in `.envl` files with the CLI, segmenting them into Unicode-compliant characters, lexing the characters into Tokens and parsing the Tokens into an Abstract Syntax Tree (AST).

Each step in the rewrite is done in a new `patch` version to ensure clarity of changes.

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