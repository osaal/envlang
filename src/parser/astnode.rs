use std::rc::Rc;
use crate::{environment::EnvScope, symbols::Operators, parser::ParserError};

#[derive(Debug, PartialEq, Clone)]
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
        value: Option<Rc<AstNode>>,     // Holds None upon initialization
        inherit: Option<Rc<AstNode>>,   // Capture the inherited elements as pointers, or none if none are inherited
    },

    // Inheritance
    Inherit {
        names: Option<Vec<Rc<str>>>,    // None for wildcard inheritance, Some for specified inheritance
    },

    // Functions
    Function {
        params: Vec<Rc<str>>,
        body: Box<AstNode>,
        inherit: Option<Rc<AstNode>>, // Capture the inherited elements as pointers, or none if none are inherited
        r#return: Rc<AstNode>,
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
            AstNode::Let { name, value , inherit: _ }
                => format!("Let {} = {} with {}", name, {
                    if let Some(val) = value {
                        val.to_string()
                    } else {
                        "nothing".to_string()
                    }
                }, {
                    match self.get_inherited_names() {
                        Some(names) => format!("inherited elements {}", {
                            names.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        }),
                        None => "no inherited elements".to_string()
                    }
                }),
            AstNode::Inherit { names }
                => format!("Inherit elements '{}'",
                    match names {
                        Some(names) => names.iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>()
                                        .join(", "),
                        None => "*".to_string()
                    }),
            AstNode::Function { params, body: _, inherit, r#return: _}
                => format!("Function with params [{}] and {}", params.join(", "), {
                    match inherit {
                        Some(elements) => {
                            if let Some(els) = elements.get_inherited_names() {
                                els.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                            } else {
                                "all elements from encapsulating environment".to_string()
                            }
                        },
                        None => "no inherited elements".to_string(),
                    }
                }),
            AstNode::FunctionCall { callee, arguments }
                => format!("Function call by {} with arguments [{}]",
                    callee.to_string(),
                    arguments.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ")
                ),
        }
    }
}

impl AstNode {
    // Check whether the node is an environment
    pub fn is_environment(&self) -> bool { matches!(self, AstNode::Environment { .. }) }

    // Get the bindings of an environment
    pub fn get_bindings(&self) -> Option<Vec<Rc<AstNode>>> {
        match self {
            AstNode::Environment { bindings, .. } => Some(bindings.to_vec()),
            _ => None,
        }
    }

    // Get the name of an environment
    pub fn get_name(&self) -> Option<Rc<str>> {
        match self {
            AstNode::Environment { name, .. } => name.clone(),
            _ => None,
        }
    }

    // Get the parent of an environment
    pub fn get_parent(&self) -> Option<Rc<AstNode>> {
        match self {
            AstNode::Environment { parent, .. } => parent.clone(),
            _ => None,
        }
    }

    // Get the scope of an environment
    pub fn get_scope(&self) -> Option<EnvScope> {
        match self {
            AstNode::Environment { scope, .. } => Some(scope.clone()),
            _ => None, // This should never occur!
        }
    }

    // Get inherited names
    pub fn get_inherited_names(&self) -> Option<Vec<Rc<str>>> {
        match self {
            AstNode::Inherit { names } => {
                if let Some(names) = names {
                    return Some(names.clone());
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    // Add inherited element to inheritance clause
    pub fn push_inherited_name(&mut self, node: Rc<str>) -> Result<(), ParserError> {
        match self{
            AstNode::Inherit { ref mut names } => {
                if let Some(names) = names {
                    names.push(node);
                    return Ok(());
                } else {
                    return Err(ParserError::WildcardAndElements(0, 0, "wrong implementation".to_string()))
                }
            },
            _ => Err(ParserError::NotInheritClause),
        }
    }

    // Generic field setting for Let statements
    // This should be extendable to other struct variants (e.g., Function or FunctionCall) as needed
    pub fn set_field<T>(&mut self, field_setter: impl FnOnce(&mut AstNode)) -> Result<(), &'static str> {
        match self {
            AstNode::Let { .. } => {
                field_setter(self);
                Ok(())
            },
            _ => Err("Not a Let statement")
        }
    }
}

#[cfg(test)]
mod tests {
}