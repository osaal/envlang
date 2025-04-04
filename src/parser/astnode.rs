use std::rc::Rc;
use crate::{symbols::Operators, parser::ParserError};

/// Enum variant representing the nodes of the Abstract Syntax Tree
/// 
/// The enum derives the traits: `Debug`, `PartialEq`, and `Clone`, and implements [`ToString`](AstNode::to_string).
#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    /// Integers are signed and system-sized
    Integer(isize),
    /// Floats are `f64`
    Float(f64),
    /// Booleans are `bool`s
    Boolean(bool),
    /// String literals are reference-counted pointers to `str`
    String(Rc<str>),
    /// Object identifiers are reference-counted pointers to `str`
    Identifier(Rc<str>),

    /// Environments are structs with three fields:
    /// * `name`: Name of environment, or `None` for anonymous environments.
    /// * `bindings`: Vector of reference-counted pointers to environment elements (as `AstNode`).
    /// * `parent`: Reference-counted pointer to parent environment, or `None` for global environment.
    Environment {
        name: Option<Rc<str>>,
        bindings: Vec<Rc<AstNode>>,
        parent: Option<Rc<AstNode>>,
    },

    /// Binary operations are structs with three fields:
    /// * `left`: Reference-counted pointer to left-hand-side operand (as `AstNode`).
    /// * `operator`: The operation (as [`Operators`]).
    BinaryOp {
        left: Rc<AstNode>,
        operator: Operators,
        right: Rc<AstNode>,
    },

    /// Assignments are structs with three fields:
    /// * `name`: Reference-counted pointer to assignment name (as `str`).
    /// * `value`: Reference-counted pointer to the assignment value (as `AstNode`), or `None` if initialized but unassigned.
    /// * `inherit`: Reference-counted pointer to the inheritance clause (as [`AstNode::Inherit`]), or `None` if no inheritance.
    Let {
        name: Rc<str>,
        value: Option<Rc<AstNode>>,
        inherit: Option<Rc<AstNode>>,
    },

    /// Inheritance clauses are single-field structs:
    /// * `names`: Vector of reference-counted pointers to inheritance element names (as `str`), or `None` if wildcard inheritance.
    Inherit {
        names: Option<Vec<Rc<str>>>,
    },

    /// Functions are structs with three fields:
    /// * `params`: Reference-counted pointer to function parameters (as [`AstNode::FunctionArgs`]).
    /// * `body`: Reference-counted pointer to function body (as [`AstNode::Environment`]).
    /// * `r#return`: Reference-counted pointer to function return statement (as [`AstNode::Environment`]).
    Function {
        params: Rc<AstNode>,
        body: Rc<AstNode>,
        r#return: Rc<AstNode>,
    },

    /// Function arguments are a single-element enum variant, with a vector or reference-counted pointers to argument identifiers (as [`AstNode::Identifier`]).
    FunctionArgs(Vec<Rc<AstNode>>),

    /// Function calls are structs with two fields:
    /// * `id`: Reference-counted pointer to the function being called (as [`AstNode::Identifier`])
    /// * `args`: Reference-counter pointer to the function arguments (as [`AstNode::FunctionArgs`])
    FunctionCall {
        id: Rc<AstNode>,
        args: Rc<AstNode>,
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
            AstNode::Function { params, .. }
                => format!("Function with params {}", params.to_string()),
            AstNode::FunctionArgs(params)
                => format!("[{}]",
                    params.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ")
                ),
            AstNode::FunctionCall { id, args }
                => format!("Function call to {} with arguments {}",
                    id.to_string(),
                    args.to_string()
                ),
        }
    }
}

impl AstNode {
    /// Checks whether the node is an environment
    pub fn is_environment(&self) -> bool { matches!(self, AstNode::Environment { .. }) }

    /// Checks whether a contained environment has a single element
    pub fn is_single_element_env(&self) -> bool {
        if let Some(bindings) = self.get_bindings() {
            if bindings.len() == 1 {
                return true;
            } else {
                return false;
            }
        }
        false
    }

    /// Gets the bindings of an environment
    pub fn get_bindings(&self) -> Option<Vec<Rc<AstNode>>> {
        match self {
            AstNode::Environment { bindings, .. } => Some(bindings.to_vec()),
            _ => None,
        }
    }

    /// Gets the name of an environment
    pub fn get_name(&self) -> Option<Rc<str>> {
        match self {
            AstNode::Environment { name, .. } => name.clone(),
            _ => None,
        }
    }

    /// Gets the parent of an environment
    pub fn get_parent(&self) -> Option<Rc<AstNode>> {
        match self {
            AstNode::Environment { parent, .. } => parent.clone(),
            _ => None,
        }
    }

    /// Gets the names of function arguments/parameters
    pub fn get_params(&self) -> Option<Vec<Rc<AstNode>>> {
        match self {
            AstNode::FunctionArgs(names) => {
                return Some(names.clone());
            },
            _ => {
                return None;
            },
        }
    }

    /// Gets names of inherited objects
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

    /// Adds an element to an inheritance clause
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

    /// Sets an enum variant field based on a given closure
    /// 
    /// This generic method may be expanded in the future to cover further `AstNode` variants.
    pub fn set_field<T>(&mut self, field_setter: impl FnOnce(&mut AstNode) -> Result<(), ParserError>) -> Result<(), ParserError> {
        match self {
            AstNode::Let { .. } | AstNode::FunctionArgs(_) => {
                field_setter(self)?;
                Ok(())
            },
            _ => Err(ParserError::ParserLogicError(0, 0)), // TODO: Make error more informative
        }
    }
}