use crate::nl::core::error::*;
use crate::nl::core::object::*;

use super::scope::Scope;
use crate::nl::core::error::*;

/// Function signature for all special form handlers.
/// The arguments are passed as-in, without being evaluated.
///
pub type SpecialForm = fn(&mut Scope, &[Object]) -> Result<Object, Error>;

/// Function signature for all eval form handlers.
/// The arguments are evaluated before being passed to the handler.
///
pub type EvalForm = fn(Vec<Object>) -> Result<Object, Error>;

/// Definition of a dynamic form (eval form defined in a source file).
/// The arguments are evaluated before being passed to the handler.
///
#[derive(Clone)]
pub struct DynamicForm {
    pub arguments: Vec<String>,
    pub body: Vec<Object>,
}

/// A binding.
///
pub enum Binding {
    /// A dynamic variable binding.
    DynamicVariable(Object),

    /// A special form binding.
    SpecialForm(SpecialForm),
    /// An eval form binding.
    EvalForm(EvalForm),
    /// A dynamic form binding.
    DynamicForm(DynamicForm),
    /// A macro form binding.
    MacroForm(),
}
