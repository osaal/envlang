# Structure of Envlang operations

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

Errors if:
- No errors at the moment; inappropriate Tokens are simply skipped

## Parse Tokens into ParsedInputTypes

- Status: NYI

The successful results from `crate::lexer::Lexer.tokenize()` should be passed to `crate::parser::parse_tokens()`.

Returns a `Vec<ParsedInputType>`, where each `ParsedInputType` is a valid Envlang data type.

Errors if:
- ???

## Further steps to be implemented...