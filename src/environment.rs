//! Environment objects
//! 
//! THIS DOCUMENTATION IS OUTDATED
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