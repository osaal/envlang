use std::rc::Rc;
use crate::{environment::EnvScope, symbols::ArithmeticOperators};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    // Literals
    Integer(isize),
    Float(f64),
    Boolean(bool),
    String(Rc<str>),
    Identifier(Rc<str>),

    // Environments
    Environment {
        name: Option<Rc<str>>, // None for anonymous
        bindings: Vec<Rc<AstNode>>,
        parent: Option<Rc<AstNode>>,
        scope: EnvScope,
    },

    // Expressions
    BinaryOp {
        left: Rc<AstNode>,
        operator: ArithmeticOperators,
        right: Rc<AstNode>,
    },

    // Bindings
    Let {
        name: Rc<str>,
        value: Rc<AstNode>,
    },

    // Inheritance
    Inherit {
        source: Rc<AstNode>,
        names: Vec<Rc<str>>,
    },

    // Functions
    Function {
        params: Vec<Rc<str>>,
        body: Box<AstNode>,
    },
    FunctionCall {
        callee: Box<AstNode>,
        arguments: Vec<Rc<AstNode>>,
    },
}

impl AstNode {
    pub fn is_environment(&self) -> bool {
        matches!(self, AstNode::Environment { .. })
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            AstNode::Identifier(name) => Some(name),
            _ => None,
        }
    }

    pub fn get_bindings(&self) -> Option<Vec<Rc<AstNode>>> {
        match self {
            AstNode::Environment { bindings, .. } => Some(bindings.to_vec()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}