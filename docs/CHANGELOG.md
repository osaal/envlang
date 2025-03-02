# Version 0

This is the Envlang alpha development version. Expect large API changes and sudden breaking.

## Version 0.5

This version entails a complete rewrite of reading in `.envl` files with the CLI, segmenting them into Unicode-compliant characters, lexing the characters into Tokens and parsing the Tokens into an Abstract Syntax Tree (AST).

Each step in the rewrite is done in a new `patch` version to ensure clarity of changes.

### Version 0.5.0

-  Added CHANGELOG.md
-  Corrected documentation of `io::read_file`
-  Added test cases to `io::read_file`