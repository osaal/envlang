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
    Comparison(ComparisonOperators),
    Logical(LogicalOperators),
    Other(OtherOperators),
}

impl ToString for Operators {
    fn to_string(&self) -> String {
        match self {
            Operators::Arithmetic(op) => op.to_string(),
            Operators::Comparison(op) => op.to_string(),
            Operators::Logical(op) => op.to_string(),
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

/// Types of comparison operators
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](ComparisonOperators::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperators {
    /// The less-than operator `<`
    LT,
    /// The less-than-or-equal operator `<=`
    LEQ,
    /// The greater-than operator `>`
    GT,
    /// The greater-than-or-equal operator `>=`
    GEQ,
    /// The equal operator `==`
    EQ,
    /// The not-equal operator `!=`
    NEQ,
}

impl ToString for ComparisonOperators {
    fn to_string(&self) -> String {
        match self {
            ComparisonOperators::LT => "<".to_string(),
            ComparisonOperators::LEQ => "<=".to_string(),
            ComparisonOperators::GT => ">".to_string(),
            ComparisonOperators::GEQ => ">=".to_string(),
            ComparisonOperators::EQ => "==".to_string(),
            ComparisonOperators::NEQ => "!=".to_string(),
        }
    }
}

/// Types of logical operators
/// 
/// The enum derives the traits `Debug`, `Clone`, `PartialEq`, and `Eq`, and implements [`ToString`](LogicalOperators::to_string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOperators {
    /// The binary AND operator `&` (U+0026)
    AND,
    /// The binary OR operator `|` (U+007C)
    OR,
    /// The unary NOT operator `!` (U+0021)
    NOT,
}

impl ToString for LogicalOperators {
    fn to_string(&self) -> String {
        match self {
            LogicalOperators::AND => "&".to_string(),
            LogicalOperators::OR => "|".to_string(),
            LogicalOperators::NOT => "!".to_string(),
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