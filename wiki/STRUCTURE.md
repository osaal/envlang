# Structure of Envlang operations

This file documents how Envlang reads in input files, lexes and parses meaningful tokens, and finally executes the interpreted command structure to produce output.

**While Envlang is in alpha development, this file can be used to track progress on the interpreter.**

**Update 2 April 2025**: The documentation is up-to-date with the rewrite of the lexer and parser.

## CLI operation and reading in an Envlang script

Usage:

```
envlang [FILE_NAME]
```

- Status: COMPLETE ✅

Passing a file to the CLI tool should call `crate::io::read_file()`.

Errors if:
- File name does not end in `.envl`
- Any other error situations defined in `std::io::ErrorKind`

## Segment Envlang script into Unicode grapheme segments

- Status: COMPLETE ✅

The successful results from `crate::io::read_file()` should be passed to `crate::unicodesegmenters::segment_graphemes()`.

Returns a `Vec<String>`, where each `String` is a valid Unicode character segment.

Errors if:
- ???

## Lex Unicode grapheme segments into Tokens

- Status: COMPLETE ✅

The successful results from `crate::unicodesegmenters::segment_graphemes()` should be passed to `crate::lexer::Lexer.tokenize()`.

Returns a `Vec<Token>`, where each `Token` is a lexed token.

All errors are wrapped in the custom `LexerError` type, which implements descriptive error messages including error contexts.

## Parse Tokens into ASTNodes

- Status: COMPLETE ✅

The successful results from `crate::lexer::Lexer.tokenize()` should be passed to `crate::parser::Parser.parse()`.

Returns an `AstNode`, where the environment represents the global environment. Each token is parsed and hierarchized into an abstract syntax tree representing the layers of environments within each other.

All errors are wrapped in the custom `ParserError` type, which implements descriptive error messages including line numbers (interpreted from new-line characters in the source file) and error contexts.

## Further steps to be implemented...