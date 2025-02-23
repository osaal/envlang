//! Envlang: An environment-based interpreted language
//! 
//! Envlang is an interpreted language based entirely around environments (sometimes called closures in other languages).
//! 
//! It is primarily intended to be used with the CLI binary, which is constructed by the [`main()`] function.
//! 
//! [`main()`]: ./fn.main.html

mod environment;
mod unicodesegmenters;
mod symbols;
mod parser;
mod io;
mod lexer;

use std::env;

/// The Envlang command-line interface binary
/// 
/// Envlang is written in `.envl` files and executed using the CLI tool.
/// 
/// # Usage
/// Once compiled and accessible from the terminal, envlang is run with a simple one-argument command:
/// ```no_run
/// envlang path/to/file.envl
/// ```
/// 
/// # Errors
/// The following exit codes are defined:
/// - `1`: No arguments given to Envlang
/// - `2`: Too many arguments given to Envlang
/// - `101`: General Rust panic (see stack trace for more information)
fn main() {
    let args: Vec<String> = env::args().collect();

    // Exit with error code 1 when there are no arguments
    if args.len() < 2 {
        eprintln!("Usage: envlang <path/to/file.envl>");
        std::process::exit(1);
    }
    
    // Exit with error code 2 when there are too many arguments
    if args.len() > 2 {
        eprintln!("Too many arguments");
        eprintln!("Usage: envlang <path/to/file.envl>");
        std::process::exit(2);
    }

    // Read in file
    let file_path = &args[1];
    let env_file = io::read_file(file_path).unwrap(); // TODO: Handle errors
    dbg!(env_file);
}
