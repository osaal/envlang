mod astnode;
mod error;

use crate::lexer::Token;
pub use astnode::AstNode;
pub use error::ParserError;

pub struct Parser {}

impl Parser {}

#[cfg(test)]
mod tests {
    use super::*;
}