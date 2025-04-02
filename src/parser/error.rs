use std::error::Error;
use std::fmt;

/// Error type for Envlang parser
/// 
/// The error types match various failure states during semantic analysis.
/// 
/// Errors always contain at least:
/// - The current parser position.
/// - The current line number (calculated by the parser).
/// - The value that caused the error.
/// 
/// Errors may optionally include information about:
/// - Expected and actual values.
/// - Attempted operation parameters.
/// 
/// Usage of the error types is documented in the [`Parser`](super::Parser).
#[derive(Debug, PartialEq)]
pub enum ParserError {
    NotANumber(usize, usize, String),           // (pos, line, value)
    MalformedNumber(usize, usize, String),      // (pos, line, value)
    InvalidOperation(usize, usize, String),     // (pos, line, value)
    BinaryOpWithNoLHS(usize, usize),            // (pos, line)
    BinaryOpWithNoRHS(usize, usize),            // (pos, line)
    WhitespaceInNumber(usize, usize, String),   // (pos, line, value)
    ParserLogicError(usize, usize),             // (pos, line)
    UnexpectedEOF(usize, usize),                // (pos, line)
    UnclosedEnvironment(usize),                 // (line)
    MissingLetIdentifier(usize, usize),         // (pos, line)
    MissingAssignmentOp(usize, usize),          // (pos, line)
    InvalidAssignmentOp(usize, usize, String),  // (pos, line, value)
    EmptyEnv(usize, usize, String),             // (pos, line, value)
    InvalidAccessionTarget(usize, usize, String), // (pos, line, value)
    InvalidAccessionSource(usize, usize, String), // (pos, line, value)
    InvalidInheritanceToken(usize, usize, String),// (pos, line, value)
    ParentlessInheritance(usize, usize, String),// (pos, line, value)
    DoubleInheritanceParen(usize, usize, String),   // (pos, line, value)
    WildcardAndElements(usize, usize, String),  // (pos, line, value)
    NotInheritClause, // TODO: I am inappropriately formatted!
    NotAnEnvironment(usize, usize, String),     // (pos, line, value)
    InvalidFunArgToken(usize, usize, String),   // (pos, line, value)
    DoubleFunArgBracket(usize, usize, String),  // (pos, line, value)
    UnclosedArgumentClause(usize),              // (line)
    MissingFunctionName(usize, usize, String),  // (pos, line, value)
    MissingFunctionArgs(usize, usize),          // (pos, line)
    MissingFunctionBody(usize, usize),          // (pos, line)
    MissingReturnStatement(usize, usize, String),   // (pos, line, value)
    InvalidTokenInFnSignature(usize, usize, String),// (pos, line, value)
    UnexpectedReturn(usize, usize),             // (pos, line)
}

impl Error for ParserError {}

/// TODOs:
/// - MalformedNumber is not informative enough, the context is not visible -> Need to reconstruct the line (through tokens or source)
/// - NotANumber is not informative enough, the context is not visible -> Need to reconstruct the line (through tokens or source)
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::NotANumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Value '{}' is not a number", line, pos, valuestr),
            ParserError::MalformedNumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Value '{}' is a malformed number", line, pos, valuestr),
            ParserError::InvalidOperation(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Invalid operation '{}'", line, pos, valuestr),
            ParserError::BinaryOpWithNoLHS(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Binary operation with no left-hand side", line, pos),
            ParserError::BinaryOpWithNoRHS(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Binary operation with no right-hand side", line, pos),
            ParserError::WhitespaceInNumber(pos, line, valuestr) => 
                write!(f, "Parser error at source line {}, token position {}: Whitespace inside number '{}'", line, pos, valuestr),
            ParserError::ParserLogicError(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Internal parser logic error", line, pos),
            ParserError::UnexpectedEOF(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Unexpected end of file", line, pos),
            ParserError::UnclosedEnvironment(line) =>
                write!(f, "Parser error at source line {}: Unclosed environment", line),
            ParserError::MissingLetIdentifier(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Missing identifier after 'let'", line, pos),
            ParserError::MissingAssignmentOp(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Missing assignment operator '='", line, pos),
            ParserError::InvalidAssignmentOp(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected assignment operator '=', not '{}'", line, pos, valuestr),
            ParserError::EmptyEnv(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Empty environment '{}'", line, pos, valuestr),
            ParserError::InvalidAccessionTarget(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Invalid accession target '{}'", line, pos, valuestr),
            ParserError::InvalidAccessionSource(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Invalid accession source '{}'", line, pos, valuestr),
            ParserError::InvalidInheritanceToken(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Invalid token '{}' in inheritance statement", line, pos, valuestr),
            ParserError::ParentlessInheritance(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Inheritance in parentless environment: '{}'", line, pos, valuestr),
            ParserError::DoubleInheritanceParen(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Second opening parenthesis for inheritance: '{}'", line, pos, valuestr),
            ParserError::WildcardAndElements(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Cannot specify both identifiers and wildcard: '{}'", line, pos, valuestr),
            ParserError::NotInheritClause =>
                write!(f, "Parser error: Attempted to push name to something else than an inherit clause"),
            ParserError::NotAnEnvironment(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected an Environment instead of: '{}'", line, pos, valuestr),
            ParserError::InvalidFunArgToken(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected whitespace, brackets, commas, or identifiers, instead of: '{}", line, pos, valuestr),
            ParserError::DoubleFunArgBracket(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Second opening bracket for function arguments: '{}'", line, pos, valuestr),
            ParserError::UnclosedArgumentClause(line) =>
                write!(f, "Parser error at source line {}: Unclosed function argument clause", line),
            ParserError::MissingFunctionName(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected function identifier instead of: '{}'", line, pos, valuestr),
            ParserError::MissingFunctionArgs(pos,line) =>
                write!(f, "Parser error at source line {}, token position {}: Expected function arguments", line, pos),
            ParserError::MissingFunctionBody(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: Expected function body", line, pos),
            ParserError::MissingReturnStatement(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected return statement instead of: '{}", line, pos, valuestr),
            ParserError::InvalidTokenInFnSignature(pos, line, valuestr) =>
                write!(f, "Parser error at source line {}, token position {}: Expected inheritance clause or assignment operator instead of: '{}'", line, pos, valuestr),
            ParserError::UnexpectedReturn(pos, line) =>
                write!(f, "Parser error at source line {}, token position {}: The `return` keyword is not valid in this context", line, pos)
        }
    }
}