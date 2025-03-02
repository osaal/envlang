//! Envlang is an interpreted programming language focused on environment manipulation
//! 
//! This crate provides both a library for working with Envlang files and a CLI tool for running Envlang scripts.

pub mod io;
pub mod unicodesegmenters;
pub mod lexer;
pub mod parser;
pub mod symbols;
pub mod environment;