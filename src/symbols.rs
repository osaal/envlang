/// Types of arithmetic operators
/// 
/// -  ADD: Addition operator `+`
/// -  SUBTRACT: Subtraction operator `-`
/// -  DIVIDE: Division operator `/`
/// -  MULTIPLY: Multiplication operator `*`
/// -  MODULUS: Remainder operator `%`
/// -  EXPONENTIATION: Exponentiation operator `^`
#[derive(Debug, Clone)]
pub enum ArithmeticOperators {
    // Arithmetic operators
    ADD,            // +
    SUBTRACT,       // -
    DIVIDE,         // /
    MULTIPLY,       // *
    MODULUS,        // %
    EXPONENTIATION, // ^
}

/// Types of generic symbols
/// 
/// -  DASH: The dash symbol "-"
/// -  UNDERSCORE: The underscore symbol "_"
#[derive(Debug, Clone)]
pub enum GenericSymbols {
    DASH,           // -
    UNDERSCORE,     // _
}

/// Types of reserved symbols
/// 
/// 
#[derive(Debug, Clone)]
pub enum ReservedSymbols {
    TERMINATOR,     // ;
    ACCESSOR,       // .
    ASSIGNMENT,     // =
    ENVOPEN,        // {
    ENVCLOSE,       // }
    INHERITOPEN,    // (
    INHERITCLOSE,   // )
    FUNARGOPEN,     // [
    FUNARGCLOSE,    // ]
}

/// Types of string-related symbols
/// 
/// -  DOUBLEQUOTE: Double quote symbol `"`
/// -  SINGLEQUOTE: Single quote symbol a.k.a. apostrophe `'`
/// -  ESCAPE: Escape character `\`
#[derive(Debug, Clone)]
pub enum StringSymbols {
    DOUBLEQUOTE,    // "
    SINGLEQUOTE,    // '
    ESCAPE,         // \
}

/// Reserved keywords
/// 
/// -  LET: Assignment keyword `let`
/// -  INHERIT: Inheritance keyword `inherit`
/// -  FUN: Function assignment keyword `fun`
#[derive(Debug, Clone)]
pub enum Keywords {
    LET,            // let
    INHERIT,        // inherit
    FUN,            // fun
}