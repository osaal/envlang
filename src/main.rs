mod environment;
mod unicodesegmenters;
mod symbols;
mod parser;
mod parsererror;
mod io;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Read in the file
    if args.len() < 2 {
        eprintln!("Usage: envlang <path/to/file.envl>");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let env_file = io::read_file(file_path).unwrap(); // TODO: Handle errors
    dbg!(env_file);
}
