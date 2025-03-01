//! Envlang-reserved symbols
//! 
//! All symbols and keywords used and/or recognised in Envlang are enumerated in this module's enums.
//! 
//! This list will grow significantly until the release of version 1.0.x.

/// Types of arithmetic operators
/// 
/// These operators are overloaded according to the [data types] being operated on.
/// 
/// -  `ADD`: Addition operator `+`
/// -  `SUBTRACT`: Subtraction operator `-`
/// -  `DIVIDE`: Division operator `/`
/// -  `MULTIPLY`: Multiplication operator `*`
/// -  `MODULUS`: Remainder operator `%`
/// -  `EXPONENTIATION`: Exponentiation operator `^`
/// 
/// The enum derives the traits `Debug` and `Clone`.
/// 
/// [data types]: ../environment/enum.EnvValue.html
#[derive(Debug, Clone)]
pub enum ArithmeticOperators {
    ADD,            // +
    SUBTRACT,       // -
    DIVIDE,         // /
    MULTIPLY,       // *
    MODULUS,        // %
    EXPONENTIATION, // ^
}

/// Types of generic symbols
/// 
/// These symbols are not used by Envlang per se, but are recognised for purposes of string parsing.
/// 
/// -  `DASH`: The dash symbol `-`
/// -  `UNDERSCORE`: The underscore symbol `_`
/// 
/// The enum derives the traits `Debug` and `Clone`.
#[derive(Debug, Clone)]
pub enum GenericSymbols {
    DASH,           // -
    UNDERSCORE,     // _
}

/// Types of reserved symbols
/// 
/// The accessor symbol is overloaded on whether it operates on environments or number-like types (integer, float).
/// 
/// - `TERMINATOR`: Line (implicit environment) terminator symbol `;`
/// - `ACCESSOR`: Environment accessor symbol `.`
/// - `ASSIGNMENT`: Environment assignment symbol `=`
/// - `ENVOPEN`: Start of explicit environment declaration symbol `{`
/// - `ENVCLOSE`: End of explicit environment declaration symbol `}`
/// - `INHERITOPEN`: Start of inheritance declaration symbol `(`
/// - `INHERITCLOSE`: End of inheritance declaration symbol `)`
/// - `FUNARGOPEN`: Start of function argument declaration symbol `[`
/// - `FUNARGCLOSE`: End of function argument declaration symbol `]`
/// 
/// The enum derives the traits `Debug` and `Clone`.
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
/// -  `DOUBLEQUOTE`: Double quote symbol `"`
/// -  `SINGLEQUOTE`: Single quote symbol a.k.a. apostrophe `'`
/// -  `ESCAPE`: Escape character `\`
/// 
/// The enum derives the traits `Debug` and `Clone`.
#[derive(Debug, Clone)]
pub enum StringSymbols {
    DOUBLEQUOTE,    // "
    SINGLEQUOTE,    // '
    ESCAPE,         // \
}

/// Reserved keywords
/// 
/// The reserved keywords may not be used as environment identifiers.
/// 
/// -  `LET`: Assignment keyword `let`
/// -  `INHERIT`: Inheritance keyword `inherit`
/// -  `FUN`: Function assignment keyword `fun`
/// 
/// The enum derives the traits `Debug` and `Clone`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keywords {
    LET,            // let
    INHERIT,        // inherit
    FUN,            // fun
}

/// Booleans
/// 
/// The booleans may not be used as identifiers.
/// 
/// -  `TRUE`: Boolean value `true`
/// -  `FALSE`: Boolean value `false`
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Booleans {
    TRUE,           // true
    FALSE,          // false
}