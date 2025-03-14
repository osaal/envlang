use std::rc::Rc;
use crate::{environment::EnvScope, symbols::Operators};

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
        operator: Operators,
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

impl ToString for AstNode {
    fn to_string(&self) -> String {
        match self {
            AstNode::Integer(num)
                => num.to_string(),
            AstNode::Float(num)
                => num.to_string(),
            AstNode::Boolean(b)
                => b.to_string(),
            AstNode::String(s)
                => s.to_string(),
            AstNode::Identifier(name)
                => name.to_string(),
            AstNode::Environment { name, .. } => {
                if let Some(name) = name {
                    format!("Environment '{}'", name)
                } else {
                    "Anonymous environment".to_string()
                }
            },
            AstNode::BinaryOp { left, operator, right }
                => format!("{} {} {}", left.to_string(), operator.to_string(), right.to_string()),
            AstNode::Let { name, value }
                => format!("Let {} = {}", name, value.to_string()),
            AstNode::Inherit { source, names }
                => format!("Inherit elements '{}' from '{}'", source.to_string(), names.join(", ")),
            AstNode::Function { params, .. }
                => format!("Function with params [{}]", params.join(", ")),
            AstNode::FunctionCall { callee, arguments }
                => format!("Function call by {} with arguments [{}]", callee.to_string(), arguments.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
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
}