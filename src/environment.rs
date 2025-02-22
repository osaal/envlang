use std::fmt;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Scope of an environment
/// 
/// - GLOBAL: Environment is accessible globally
/// - LOCAL: (default) Environment is accessible locally within its defined scope
/// - INHERITED: Environment is accessible locally and within the nested environment created with the `inherit` keyword
#[derive(Debug, Clone)]
pub enum EnvScope {
    GLOBAL,
    LOCAL,
    INHERITED,
}

impl Display for EnvScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvScope::GLOBAL => write!(f, "Global"),
            EnvScope::INHERITED => write!(f, "Inherited from parent"),
            EnvScope::LOCAL => write!(f, "Local"),
        }
    }
}

/// Value of an environment
/// 
/// - INT: Integer value, implemented as `i32` in Rust
/// - FLOAT: Floating-point value, implemented as `f64` in Rust
/// - BOOL: Boolean value, implemented as `bool` in Rust
/// - STRING: String value, implemented as `String` in Rust
/// - ENVSP: Environment smart pointer value, implemented as `Box<Environment>` in Rust
#[derive(Debug, Clone)]
pub enum EnvValue {
    INT(i32),
    FLOAT(f64),
    BOOL(bool),
    STRING(String),
    ENV(Rc<Environment>),
}

impl Display for EnvValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvValue::INT(val) => write!(f, "Integer: {}", val),
            EnvValue::FLOAT(val) => write!(f, "Float: {}", val),
            EnvValue::BOOL(val) => write!(f, "Bool: {}", val),
            EnvValue::STRING(val) => write!(f, "String: {}", val),
            EnvValue::ENV(val) => write!(f, "Environment name: {}", val.get_name()),
        }
    }
}

/// Name of an environment
/// 
/// - STRING: String name of environment, implemented as `String` in Rust
/// - ANON: Anonymous environment
#[derive(Debug, Clone)]
pub enum EnvName {
    STRING(String),
    ANON,
}

impl Display for EnvName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvName::STRING(val) => write!(f, "{}", val),
            EnvName::ANON => write!(f, "Anonymous environment"),
        }
    }
}

/// Struct for defining Environment arguments
/// 
pub struct EnvironmentConfig {
    pub parent: Option<Rc<Environment>>,
    pub name: EnvName,
    pub scope: EnvScope,
    pub elements: RefCell<Vec<EnvValue>>
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            parent: None,
            name: EnvName::ANON,
            scope: EnvScope::LOCAL,
            elements: RefCell::new(Vec::new()),
        }
    }
}

/// The Environment base model
/// 
/// - Parent: A smart pointer pointing to the parent environment within which the `Environment` resides
/// - Name: Either a name as `EnvName::STRING` or `EnvName::ANON` for anonymous environments
/// - Scope: A scope value determined by the `EnvScore` enum
/// - Elements: A vector of smart pointers pointing to zero or more `EnvValue` enums
#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Weak<Environment>>,
    name: EnvName,
    scope: EnvScope,
    elements: RefCell<Vec<EnvValue>>
}

impl Environment {
    /// Create new Environment
    pub fn new(options: EnvironmentConfig) -> Rc<Self> {
        Rc::new(Self {
            parent: options.parent.as_ref().map(Rc::downgrade),
            name: options.name,
            scope: options.scope,
            elements: options.elements,
        })
    }
    /// Get pointer to parent `Environment`
    /// 
    /// If the current `Environment` is the top level, this will return `None`
    /// 
    /// If the parent exists, it returns an upgraded Rc reference from the original Weak reference
    pub fn get_parent(&self) -> Option<Rc<Environment>> {
        return self.parent.as_ref().and_then(|weak_parent| weak_parent.upgrade())
    }

    /// Get vector of enclosed `EnvValue`s, which themselves may be data values or `Environment`s
    /// 
    /// Transfers ownership of vector to calling context
    pub fn get_elements(&self) -> Vec<EnvValue> {
        return self.elements.borrow().clone()
    }

    /// Get pointer to environment scope as `EnvScope`
    pub fn get_scope(&self) -> &EnvScope {
        return &self.scope
    }

    /// Get pointer to environment name as `EnvName`
    pub fn get_name(&self) -> &EnvName {
        return &self.name
    }

    /// Add an element to the vector of elements
    pub fn add_element(parent: &Rc<Environment>, child: EnvValue) {
        parent.elements.borrow_mut().push(child)
    }
}

// Unit tests for environment.rs
#[cfg(test)]
mod tests {
    use super::*;
}