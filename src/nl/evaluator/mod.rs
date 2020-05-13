mod binding;

pub use super::core::error::*;
pub use super::core::location::*;
pub use super::core::object::*;
pub use super::reader::*;

pub use binding::*;

use std::collections::HashMap;

/// Core object used for evaluating NeoLisp code.
///
pub struct Evaluator {
    stack: Vec<State>,
}

/// Mode of the evaluator.
///
pub enum Mode {
    /// Inherits the mode of the parent state.
    Inherited,
    /// Regular evaluation mode. The return of a function is just passed to the engine.
    Evaluate,
    /// Special evaluation mode for macros. The return of the function is used as actual
    /// code and is reevaluated by the engine.
    Expansion,
}

/// State modifier.
///
enum Modifier {
    /// Used when calling functions.
    Call { returned: Option<Object> },
    /// Used by loops.
    Loop { broke: bool },
}

/// An evaluation state.
///
struct State {
    /// Modifier of the state.
    modifier: Option<Modifier>,
    /// Evaluation mode of the state.
    mode: Mode,
    /// Bindings done in the state.
    bindings: HashMap<String, Binding>,
}
