mod environment;
mod unicodesegmenters;
mod symbols;
mod parser;
mod parsererror;
mod io;

use std::env;

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
