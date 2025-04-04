# Envlang: An interpreted language based on environments

Envlang is an interpreted language written in Rust, whose main feature is treating all input as environments. It draws heavy inspiration from `R`, especially from its functional side.

Environments are a special data type that can function as expressions, data values, data containers, and even functions.

Envlang is dynamically typed, constantly assigned, and functionally pure.

## Current development status

Envlang is under alpha development. The language is not fully functional currently, but can be installed for testing purposes.

Currently, the lexing and parsing stages are fully functional. There are several missing language features, such as control flow and basic I/O.

The release will be bumped to 1.x.x for the stable release version.

## Installation

Install Envlang with `cargo`:

```
git clone https://github.com/osaal/envlang
cd envlang
cargo build
```

## Usage

Envlang can be used as a binary CLI tool, taking a `.envl` file as its input, or through `cargo`:

```
# Alternative 1
cargo build
/path/to/binary envlang filename.envl

# Alternative 2
cargo run -- filename.envl
```

## Documentation

I aim to document the entire codebase (bar test suites), including private objects and methods. Docs can be rendered locally with `cargo`:

```
cargo doc --document-private-items
```

If you are interested in contributing to Envlang, please make sure to thoroughly document your code using docstrings.

## Syntax and Other Docs

The syntax is documented in the [Basics Markdown file](docs/Basics.md) of this repository.

The [docs](docs/) directory contains other general documentation, such as the changelog, and the structure of the Envlang interpreter. Please note, that any documentation files residing in `/docs` may currently be out-of-date.

Future versions may move these docs to a dedicated page.

## Issues

If you have issues running Envlang, firstly: This is to be expected, considering the very early development cycle of Envlang.

Secondly, please leave a Github Issue describing your issue in detail! Please add at least the following information:

-  Actions: What did you do?
-  Expected behaviour: What did you expect to occur?
-  Actual behaviour: What actually happened?
-  OS: Which operating system and version were you running?
    -  For Linux: Please include distribution version and/or kernel version.
    -  For Windows: Please include the Windows version (8/10/11 etc.) and OS build version.
    -  For Mac OSX: I don't know, include something relevant about Macs?
-  Toolchain: Which versions of `cargo` and `rustc` were you running?
    -  `rustc --version`
    -  `cargo --version`

## Improvements and Suggestions

You can contribute to Envlang by leaving a Pull Request or starting a Discussion on a potential change.

Please note, that Pull Requests are subject to my whims and wishes - your idea might not be bad, just not to my liking (purely object-oriented fans, please do not interact, thanks!).

If you can't/won't implement the suggestion yourself, you can always start a Discussion with suggestions as to what the change means, how it should work, and why it is necessary.