# Envlang: An interpreted language based on environments

Envlang is an interpreted language written in Rust, whose main feature is treating all input as environments.

Environments are a special data type in Envlang, who can function as expressions, data values, data containers, and even functions.

Envlang is:

-  Statically typed (NYI) with constant assignments
-  Functionally pure -- all declared functions take an input and return an output, with no side-effects allowed

## Current development status

Envlang is under alpha development. The language is not fully functional currently, but can be installed for testing purposes.

The release will be bumped to 1.x.x for the stable release version.

## Installation and Usage

Install Envlang with `cargo`:

```
git clone https://github.com/osaal/envlang
cd envlang
cargo build
```

Envlang can be used as a binary CLI tool, taking a `.envl` file as its input, or through `cargo`:

```
# Alternative 1
cargo build
/path/to/binary envlang filename.envl

# Alternative 2
cargo run -- filename.envl
```

## Syntax

The syntax is documented and updated in the [Basics Markdown file](docs/Basics.md) of this repository. Future versions may move to a dedicated page.

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