# New Structure (a rewrite of internal representation of Envlang code)

1. The raw input is made Unicode-compliant with `unicodesegmenters.rs` --> Outputs a vector of Strings
    - Output is guaranteed to be Unicode-segmented
2. The Unicode-compliant String vector is lexed with `lexer.rs` --> Outputs a vector of Tokens
    - Output is guaranteed to be valid Envlang tokens without meaning or reasonable combinations
    - E.g.: numbers are still separate, even if they occur one after another (and should be concatenated into larger structures)
3. The Token vector is parsed into an abstract syntax tree with `parser.rs` --> Outputs a nested ASTNode structure
    - Output is guaranteed to be syntactically valid
    - Reasonable combinations are collapsed
    - E.g.: Subsequent numbers become integers or floats, operators are subsumed by Operations, which have left- and right-hand sides as appropriate
4. The AST is analyzed into a semantically valid and meaningful intermediate representation with `analyzer.rs` --> Outputs a nested IR structure
    - Output is guaranteed to be semantically valid and ready for execution
    - All operations have been validated to be doable (types, handedness, etc.)
    - All scopes have been cleared (in-scopeness, out-of-scopeness, shadowing and masking, etc.)
5. The IR is executed 
    - Output is guaranteed to be "interpretation-time" valid, but can still runtime-error
    - Output goes onto screen as appropriate

## Checklist of rewrite

- [x] IO operations (finished in version 0.5.0)
    - [x] Implementation
    - [x] Tests
    - [x] Documentation
- [x] Unicode segmentation (finished in version 0.5.1)
    - [x] Implementation
    - [x] Tests
    - [x] Documentation
- [x] Lexer (finished in version 0.5.2)
    - [x] Implementation
    - [x] Tests
    - [x] Documentation
- [x] Parser (finished in version 0.5.11)
    - [x] Implementation
    - [x] Tests
    - [x] Documentation
- [ ] Analyzer
    - [ ] Implementation
    - [ ] Tests
    - [ ] Documentation
- [ ] Executer
    - [ ] Implementation
    - [ ] Tests
    - [ ] Documentation

## Rewriting process

The rewrite of structure steps 1-3 will happen in one branch. Increment version to a new minor version to indicate API breakage.

Start walking the new structure from the very start. For each element in the new structure:
1. Increment patch to a new version.
2. Refactor old implementations with the prefix OLD_
3. Refactor old tests with the prefix OLD_
4. Refactor old errors with the prefix OLD_
5. Write the new implementation
6. Test the implementation
7. Document the implementation

If the implementation is very large, split steps 5-7 into substeps, incrementing patch in-between documenting and writing.

Once the rewrite of structure steps 1-3 is done, merge onto main.

Create a new branch for the IR (structure step 4). Increment version to a new minor version to indicate API changes (new APIs).

Follow steps 5-7 again, splitting into substeps with patch version incrementations if necessary.

Once the IR is complete, merge onto main.

Create a new branch for the execution (structure step 5). Increment version to a new minor version to indicate API changes (new APIs).

Once the execution step is complete, merge onto main.

