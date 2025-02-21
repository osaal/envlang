use crate::operators::Operators;
use unicode_segmentation::Graphemes;

/// Types of parsed input
/// 
/// -  INT: Integer of type `i32`
/// -  FLOAT: Floating-point value of type `f64`
/// -  STRING: String of type `String`
/// -  BOOL: Boolean of type `bool`
/// -  OPERATOR: Operator of `Operators` enum
#[derive(Debug, Clone)]
pub enum ParsedInputType {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
    OPERATOR(Operators),
}

// GOAL:
// Step through each element in a Unicode-parsed Vec<String>
// For each element, check which type it should become of ParsedInputType
// Segment the elements into appropriate types and return them as a vector

pub fn walk_input(input: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for (idx, el) in input.iter().enumerate() {
        if let Ok(int) = el.parse::<i32>() {
            // We know that it conforms to i32
            // It is either a single integer, a digit part of a larger integer, or a digit part of a float

            let start_idx = idx;                // Capture the first integer's index
            let mut temp: String = String::new();      // Create a temp string to store following integers
            temp.push_str(el);                  // Push the first integer as char to temp string

            // We start walking the following input and check each time if it also conforms to i32
            for (next_idx, next_el) in input[start_idx+1..].iter().enumerate() {
                if let Ok(int) = next_el.parse::<i32>() {
                    temp.push_str(next_el);     // Also an int, append to temporary string
                } else {
                    // The next element does not conform to an int
                    // We should be done with constructing an int
                    // However, it could still have been a float!
                    let result = temp.parse::<i32>().unwrap();
                }
            }
        } else {
            // We know that it does not conform to i32
            // It is a character value of some kind
            // Could be alphabetical or symbolic
        }
    }

    return output;
}

pub fn parse_float(integers: i32, rest: String) -> f64 {
    for (idx, el) in rest.chars().enumerate() {
        if let Some(val) = el.to_digit(10) {
            // Value at or after decimal point still parses to i32, so it is a decimal
            // TODO: Big problem! to_digit returns u32
            // Handling chars in general is bad, since we might be messing with the Unicode segmentation
            // Perhaps change code to function on a vector of Graphemes from crate::unicodeparsers::parse_into_graphemes()?
        } else {
            // Value does not parse to i32
            // It is either the decimal point or something else
            // If decimal: Skip and continue
            // If something else: Finish up
        }
    }

    return 0.0f64;
}

pub fn is_alphanumeric(vector: Vec<String>) -> bool {


    return false;
}