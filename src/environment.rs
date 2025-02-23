//! Environment objects
//! 
//! The environment is the building block of Envlang.
//! 
//! [`Environment`] structs are constructed by the [parser] according to the grammar of Envlang.
//! 
//! To enable default values, environments are constructed with pre-specified [`EnvironmentConfig`] structs.
//! 
//! Environment data is encapsulated in three enumerations: [`EnvName`], [`EnvScope`], and [`EnvValue`].
//! 
//! [`Environment`]: ./struct.Environment.html
//! [parser]: ../parser/index.html
//! [`EnvironmentConfig`]: ./struct.EnvironmentConfig.html
//! [`EnvName`]: ./enum.EnvName.html
//! [`EnvScope`]: ./enum.EnvScope.html
//! [`EnvValue`]: ./enum.EnvValue.html

use std::fmt;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Scope of an environment
/// 
/// - `LOCAL`: (default) Environment is accessible locally within its defined scope
/// - `GLOBAL`: Environment is accessible globally
/// - `INHERITED`: Environment is accessible locally and within the nested environment created with the [`inherit`] keyword
/// 
/// The scope derives the traits `Debug`, `Clone` and `PartialEq`, and implements `Display`.
/// 
/// [`inherit`]: ../symbols/enum.Keywords.html
#[derive(Debug, Clone, PartialEq)]
pub enum EnvScope {
    LOCAL,
    GLOBAL,
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
/// - `INT`: Integer value, implemented as `isize` in Rust
/// - `FLOAT`: Floating-point value, implemented as `f64` in Rust
/// - `BOOL`: Boolean value, implemented as `bool` in Rust
/// - `STRING`: String value, implemented as `String` in Rust
/// - `ENV`: Environment smart pointer value, implemented as `Rc<Environment>` in Rust
/// 
/// The value derives the traits `Debug`, `Clone` and `PartialEq`, and implements `Display`.
/// 
/// # Errors
/// These values are constructed by the [`parser`], and any errors should be handled by the parser itself
/// 
/// [`parser`]: ../parser/index.html
#[derive(Debug, Clone, PartialEq)]
pub enum EnvValue {
    INT(isize),
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
/// - `STRING`: String name of environment, implemented as `String` in Rust
/// - `ANON`: Anonymous environment
/// 
/// The name derives the traits `Debug`, `Clone` and `PartialEq`, and implements `Display`.
#[derive(Debug, Clone, PartialEq)]
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
/// See [`Environment`] for a listing of members.
/// 
/// The environment implements the `Default` trait with the following defaults:
/// - `parent`: `None`
/// - `name`: `EnvName::ANON`
/// - `scope`: `EnvName::LOCAL`
/// - `elements`: `RefCell::new(Vec::new())`
/// 
/// [`Environment`]: ./struct.Environment.html 
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
/// - `parent`: An `Option<T>` containg a reference-counted pointer to the parent `Environment`. The global environment is given `None`
/// - `name`: An [`EnvName`] comprising the name of the environment, or if it is an anonymous environment
/// - `scope`: An [`EnvScope`] comprising the scope of the environment
/// - `elements`: A vector of [`EnvValue`]s referenced to by a smart mutable reference pointer, containing the elements of the environment
/// 
/// The environment derives the traits `Debug` and `Clone`, and implements `PartialEq`
/// 
/// [`EnvName`]: ./enum.EnvName.html
/// [`EnvScope`]: ./enum.EnvScope.html
/// [`EnvValue`]: ./enum.EnvValue.html
#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Weak<Environment>>,
    name: EnvName,
    scope: EnvScope,
    elements: RefCell<Vec<EnvValue>>
}

/// Partial equality checking for Environments
/// 
/// Partial equality is achieved when the names, scopes, parentage, and number and enumeration of elements all match.
/// 
/// Same parentage is achieved when both parents point to the same allocation.
/// 
/// The equality checking will exit early with the following order:
/// 1. Name and scope
/// 2. Number of elements
/// 3. Both have parents
/// 4. Same parents
/// 5. All same elements
impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name || self.scope != other.scope {
            return false;
        }

        if self.elements.borrow().len() != other.elements.borrow().len() {
            return false;
        }

        let self_has_parent = self.parent.is_some();
        let other_has_parent = other.parent.is_some();

        if self_has_parent != other_has_parent {
            return false;
        }

        if self_has_parent {
            if let (Some(self_parent), Some(other_parent)) = (self.get_parent(), other.get_parent()) {
                if !Rc::ptr_eq(&self_parent, &other_parent) {
                    return false;
                }
            }
        }

        self.elements
            .borrow()
            .iter()
            .zip(other.elements.borrow().iter())
            .all(|(a, b)| a == b)
    }
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
    pub fn add_element(&self, child: EnvValue) {
        self.elements.borrow_mut().push(child)
    }
}

// Unit tests for environment.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_equality() {
        let env1 = Environment::new(EnvironmentConfig {
            name: EnvName::STRING("test".to_string()),
            ..Default::default()
        });
        let env2 = Environment::new(EnvironmentConfig {
            name: EnvName::STRING("test".to_string()),
            ..Default::default()
        });

        assert_eq!(env1.name, env2.name);
        assert_eq!(env1.scope, env2.scope);
        assert_eq!(*env1, *env2);
    }
}