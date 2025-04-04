//! Envlang-reserved symbols
//! 
//! All symbols and keywords used and/or recognised in Envlang are enumerated in this module's enums.
//! 
//! This list will grow significantly until the release of version 1.0.0.

/// Types of operators
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](Operators::to_string).
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
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](ArithmeticOperators::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticOperators {
    /// The `+` operator (U+002B)
    ADD,
    /// The `-` operator (U+002D)
    SUBTRACT,
    /// The `/` operator (U+002F)
    DIVIDE,
    /// The `*` operator (U+002A)
    MULTIPLY,
    /// The `%` operator (U+0025)
    MODULUS,
    /// The `^` operator (U+005E)
    EXPONENTIATION,
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
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](OtherOperators::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtherOperators {
    /// The `.` operator (U+002E)
    ACCESSOR,
    /// The `=` operator (U+003D)
    ASSIGNMENT,
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
/// The enum derives the traits `Debug`, and `Clone`.
#[derive(Debug, Clone)]
pub enum GenericSymbols {
    /// The `-` symbol (U+002D)
    DASH,
    /// The `_` symbol (U+005F)
    UNDERSCORE,
}

/// Types of reserved symbols
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq` and `Eq`, and implements [`ToString`](ReservedSymbols::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservedSymbols {
    /// The `;` symbol (U+003B)
    TERMINATOR,
    /// The `{` symbol (U+007B)
    ENVOPEN,
    /// The `}` symbol (U+007D)
    ENVCLOSE,
    /// The `(` symbol (U+0028)
    INHERITOPEN,
    /// The `)` symbol (U+0029)
    INHERITCLOSE,
    /// The `[` symbol (U+005B)
    FUNARGOPEN,
    /// The `]` symbol (U+005D)
    FUNARGCLOSE,
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
/// The enum derives the traits `Debug`, and `Clone`.
#[derive(Debug, Clone)]
pub enum StringSymbols {
    /// The `"` symbol (U+0022)
    DOUBLEQUOTE,
    /// The `'` symbol (U+0027)
    SINGLEQUOTE,
    /// The `\\` symbol (U+005C)
    ESCAPE,
}

/// Reserved keywords
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](Keywords::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keywords {
    /// The "let" keyword
    LET,
    /// The "inherit" keyword
    INHERIT,
    /// The "fun" keyword
    FUN,
    /// The "return" keyword
    RETURN,
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        match self {
            Keywords::LET => "let".to_string(),
            Keywords::INHERIT => "inherit".to_string(),
            Keywords::FUN => "fun".to_string(),
            Keywords::RETURN => "return".to_string(),
        }
    }
}

/// Booleans
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](Booleans::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Booleans {
    /// The boolean value `true`
    TRUE,
    /// The boolean value `false`
    FALSE,
}

impl ToString for Booleans {
    fn to_string(&self) -> String {
        match self {
            Booleans::TRUE => "true".to_string(),
            Booleans::FALSE => "false".to_string()
        }
    }
}