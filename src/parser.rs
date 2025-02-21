/// Types of parsed input
/// 
/// -  INT: Integer of type `i32`
/// -  FLOAT: Floating-point value of type `f64`
/// -  STRING: String of type `String`
/// -  BOOL: Boolean of type `bool`
#[derive(Debug, Clone)]
pub enum ParsedInputType {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
}

// Parsers that check if a given input String/string slice conforms to valid syntax

// Parsers that read given input String/string slices and return parsed inputs
