use std::rc::Rc;
use crate::environment::EnvScope;

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
        operator: String,
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

}

#[cfg(test)]
mod tests {
    use super::*;
}