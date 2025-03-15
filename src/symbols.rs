//! Envlang-reserved symbols
//! 
//! All symbols and keywords used and/or recognised in Envlang are enumerated in this module's enums.
//! 
//! This list will grow significantly until the release of version 1.0.x.

/// Types of operators
/// 
/// - `Arithmetic`: Arithmetic operators
/// - `Other`: Other operators
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements `ToString`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operators {
    Arithmetic(ArithmeticOperators),
    Other(OtherOperators),
}

impl ToString for Operators {
    fn to_string(&self) -> String {
        match self {
            Operators::Arithmetic(op) => op.to_string(),
            Operators::Other(op) => op.to_string(),
        }
    }
}

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
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements `ToString`.
/// 
/// [data types]: ../environment/enum.EnvValue.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticOperators {
    ADD,            // +
    SUBTRACT,       // -
    DIVIDE,         // /
    MULTIPLY,       // *
    MODULUS,        // %
    EXPONENTIATION, // ^
}

impl ToString for ArithmeticOperators {
    fn to_string(&self) -> String {
        match self {
            ArithmeticOperators::ADD => "+".to_string(),
            ArithmeticOperators::SUBTRACT => "-".to_string(),
            ArithmeticOperators::DIVIDE => "/".to_string(),
            ArithmeticOperators::MULTIPLY => "*".to_string(),
            ArithmeticOperators::MODULUS => "%".to_string(),
            ArithmeticOperators::EXPONENTIATION => "^".to_string(),
        }
    }
}

/// Types of other operators
/// 
/// The accessor operator is overloaded as the decimal point in number-like types (integer, float).
/// 
/// - `ACCESSOR`: Environment accessor symbol `.`
/// - `ASSIGNMENT`: Environment assignment symbol `=`
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements `ToString`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtherOperators {
    ACCESSOR,       // .
    ASSIGNMENT,     // =
}

impl ToString for OtherOperators {
    fn to_string(&self) -> String {
        match self {
            OtherOperators::ACCESSOR => ".".to_string(),
            OtherOperators::ASSIGNMENT => "=".to_string(),
        }
    }
}

/// Types of generic symbols
/// 
/// These symbols are not used by Envlang per se, but are recognised for purposes of string parsing.
/// 
/// -  `DASH`: The dash symbol `-`
/// -  `UNDERSCORE`: The underscore symbol `_`
/// 
/// The enum derives the traits `Debug`, and `Clone`.
#[derive(Debug, Clone)]
pub enum GenericSymbols {
    DASH,           // -
    UNDERSCORE,     // _
}

/// Types of reserved symbols
/// 
/// - `TERMINATOR`: Line (implicit environment) terminator symbol `;`
/// - `ENVOPEN`: Start of explicit environment declaration symbol `{`
/// - `ENVCLOSE`: End of explicit environment declaration symbol `}`
/// - `INHERITOPEN`: Start of inheritance declaration symbol `(`
/// - `INHERITCLOSE`: End of inheritance declaration symbol `)`
/// - `FUNARGOPEN`: Start of function argument declaration symbol `[`
/// - `FUNARGCLOSE`: End of function argument declaration symbol `]`
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq` and `Eq`, and implements `ToString`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservedSymbols {
    TERMINATOR,     // ;
    ENVOPEN,        // {
    ENVCLOSE,       // }
    INHERITOPEN,    // (
    INHERITCLOSE,   // )
    FUNARGOPEN,     // [
    FUNARGCLOSE,    // ]
}

impl ToString for ReservedSymbols {
    fn to_string(&self) -> String {
        match self {
            ReservedSymbols::TERMINATOR => ";".to_string(),
            ReservedSymbols::ENVOPEN => "{".to_string(),
            ReservedSymbols::ENVCLOSE => "}".to_string(),
            ReservedSymbols::INHERITOPEN => "(".to_string(),
            ReservedSymbols::INHERITCLOSE => ")".to_string(),
            ReservedSymbols::FUNARGOPEN => "[".to_string(),
            ReservedSymbols::FUNARGCLOSE => "]".to_string(),
        }
    }
}

/// Types of string-related symbols
/// 
/// -  `DOUBLEQUOTE`: Double quote symbol `"`
/// -  `SINGLEQUOTE`: Single quote symbol a.k.a. apostrophe `'`
/// -  `ESCAPE`: Escape character `\`
/// 
/// The enum derives the traits `Debug`, and `Clone`.
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
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements `ToString`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keywords {
    LET,            // let
    INHERIT,        // inherit
    FUN,            // fun
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        match self {
            Keywords::LET => "let".to_string(),
            Keywords::INHERIT => "inherit".to_string(),
            Keywords::FUN => "fun".to_string(),
        }
    }
}

/// Booleans
/// 
/// The booleans may not be used as identifiers.
/// 
/// -  `TRUE`: Boolean value `true`
/// -  `FALSE`: Boolean value `false`
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements `ToString`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Booleans {
    TRUE,           // true
    FALSE,          // false
}

impl ToString for Booleans {
    fn to_string(&self) -> String {
        match self {
            Booleans::TRUE => "true".to_string(),
            Booleans::FALSE => "false".to_string()
        }
    }
}