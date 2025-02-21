/// Types of operators
/// 
/// # Notes
/// The caller is responsible for parsing the syntax correctly, this enum does not care about correct parsing.
/// 
/// # Arithmetic operators
/// -  ADD: Addition operator `+`
/// -  SUBTRACT: Subtraction operator `-`
/// -  DIVIDE: Division operator `/`
/// -  MULTIPLY: Multiplication operator `*`
/// -  MODULUS: Remainder operator `%`
/// -  EXPONENTIATION: Exponentiation operator `^`
/// -  PARENSTART: Start of parenthetical expression `(`
/// -  PARENEND: End of parenthetical expression `)`
/// 
/// # Numbers
/// -  DECIMAL: Decimal point with `.` after integer
/// 
/// # Text
/// -  DOUBLEQUOTE: Start or end of string `"`
/// 
/// # Control flow
/// -  STATEMENTEND: End of single statement `;`
/// -  ENVSTART: Start of environment declaration `{`
/// -  ENVEND: End of environment declaration `}`
/// -  LET: Environment declaration keyword `let`
/// -  INHERIT: Environment inheritance keyword `inherit`
/// -  INHERITOPTIONS: Assigned environment elements for inheritance with `()` after `inherit`
/// -  FUN: Function environment declaration keyword `fun`
/// -  FUNARGS: Declared arguments for function environment with `()` after `fun`
/// -  ENVACCESSOR: Accessor operator `.` after environment call
#[derive(Debug, Clone)]
pub enum Operators {
    // Arithmetic operators
    ADD,            // +
    SUBTRACT,       // -
    DIVIDE,         // /
    MULTIPLY,       // *
    MODULUS,        // %
    EXPONENTIATION, // ^
    PARENSTART,     // (
    PARENEND,       // )
    // Numbers
    DECIMAL,        // . after integer type
    // Text
    DOUBLEQUOTE,    // "
    // Control flow
    STATEMENTEND,   // ;
    ENVSTART,       // {
    ENVEND,         // }
    LET,            // let
    INHERIT,        // inherit
    INHERITOPTIONS, // () after inherit
    FUN,            // fun
    FUNARGS,        // () after fun
    ENVACCESSOR,    // . after env type
}