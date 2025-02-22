mod environment;
mod unicodesegmenters;
mod symbols;
mod parser;
mod errortypes;
use crate::environment::*;
use crate::unicodesegmenters::*;
use std::fs::read_to_string;

fn main() {

    let test = Environment::new(EnvironmentConfig {
        name: EnvName::STRING("Test".to_string()),
        .. Default::default()
    });

    let child1 = Environment::new(EnvironmentConfig {
        parent: Some(test.clone()),
        name: EnvName::STRING("Child 1".to_string()),
        .. Default::default()
    });

    let child2 = Environment::new(EnvironmentConfig {
        parent: Some(test.clone()),
        name: EnvName::STRING("Child 2".to_string()),
        scope: EnvScope::INHERITED,
        .. Default::default()
    });

    let child3 = 5;

    Environment::add_element(
        &test,
        EnvValue::ENV(child1.clone())
    );

    Environment::add_element(
        &test,
        EnvValue::ENV(child2.clone())
    );

    Environment::add_element(
        &test,
        EnvValue::INT(child3)
    );
    println!("Created a new Environment:");
    println!("Parent: {:?}", test.get_parent());
    println!("Name: {}", test.get_name());
    println!("Scope: {}", test.get_scope());
    println!("Elements:");

    for child in test.get_elements() {
        match child {
            EnvValue::ENV(val) => println!("Child is an Environment with name: {}", val.get_name()),
            EnvValue::INT(val) => println!("Child is an integer: {}", val),
            EnvValue::FLOAT(val) => println!("Child is a float: {}", val),
            EnvValue::BOOL(val) => println!("Child is a bool: {}", val),
            EnvValue::STRING(val) => println!("Child is a string: {}", val),
        }
    }

    let input = read_to_string("test.el");
    match input {
        Ok(val) => {
            let parsed: Vec<String> = segment_graphemes(val);
            println!("Input:");
            println!("{:?}", parsed)
        },
        Err(_) => println!("Error!"),
    }
}
