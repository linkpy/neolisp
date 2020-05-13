mod binding;
mod evaluation;

pub use super::core::error::*;
pub use super::core::location::*;
pub use super::core::object::*;
pub use super::reader::*;

pub use binding::*;
pub use evaluation::*;

use std::collections::HashMap;

/// Core object used for evaluating NeoLisp code.
///
pub struct Evaluator {
    stack: Vec<State>,
}

/// Result of an evaluation.
///
pub enum Command {
    /// The evaluation succeed.
    Value(Object),
    /// An error occurred.
    Error,
    /// A `return` statement was encountered inside of a function call.
    EndCall(Object),
    /// A `break` statement was encountered using of a loop.
    EndLoop(Object),
}

/// Mode of the evaluator.
///
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    /// Inherits the mode of the parent state.
    Inherited,
    /// Regular evaluation mode. The return of a function is just passed to the engine.
    Evaluate,
    /// Special evaluation mode for macros during source code evaluation. The return of the function
    /// is used as actual code and is evaluated by the engine once the macro finishes.
    EvaluatedExpansion,
    /// Special evaluation mode for macros. The return of the function is just added to the
    /// object-tree.
    Expansion,
}

/// State marker.
///
enum Mark {
    /// Absence of mark.
    None,
    /// Used when calling functions.
    Call,
    /// Used by loops.
    Loop,
}

/// An evaluation state.
///
struct State {
    /// Optional mark of the state.
    mark: Mark,
    /// Evaluation mode of the state.
    mode: Mode,
    /// Bindings done in the state.
    bindings: HashMap<String, Binding>,
}

//==================================================================================================
// Implementations

impl Evaluator {
    /// Creates a new evaluator.
    ///
    pub fn new() -> Evaluator {
        Evaluator { stack: Vec::new() }
    }

    /// Checks if the evaluator has no state.
    ///
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Checks if the evaluator is within a function call.
    ///
    pub fn is_in_call(&self) -> bool {
        self.enforce_non_empty_stack();

        for state in self.stack.iter().rev() {
            match &state.mark {
                Mark::Call => return true,
                _ => {}
            }
        }

        false
    }

    /// Checks if the evaluator is within a loop.
    /// Returns false if it encounters a Call first.
    ///
    pub fn is_in_loop(&self) -> bool {
        self.enforce_non_empty_stack();

        for state in self.stack.iter().rev() {
            match state.mark {
                Mark::Loop => return true,
                Mark::Call => return false,
                _ => {}
            }
        }

        false
    }

    /// Gets the current mode of the evaluator.
    ///
    pub fn mode(&self) -> Mode {
        self.enforce_non_empty_stack();

        for state in self.stack.iter().rev() {
            if state.mode != Mode::Inherited {
                return state.mode;
            }
        }

        panic!("no state defines a mode !");
    }

    /// Checks if a binding exists.
    ///
    pub fn has_binding(&self, name: &str) -> bool {
        self.enforce_non_empty_stack();

        for state in self.stack.iter().rev() {
            if state.bindings.contains_key(name) {
                return true;
            }
        }

        false
    }

    /// Gets a binding from the closest state.
    ///
    pub fn get_binding(&self, name: &str) -> Option<&Binding> {
        self.enforce_non_empty_stack();

        for state in self.stack.iter().rev() {
            match state.bindings.get(name) {
                None => {}
                x => return x,
            }
        }

        None
    }

    /// Gets a binding from the closest state.
    ///
    pub fn get_binding_mut(&mut self, name: &str) -> Option<&mut Binding> {
        self.enforce_non_empty_stack();

        for state in self.stack.iter_mut().rev() {
            match state.bindings.get_mut(name) {
                None => {}
                x => return x,
            }
        }

        None
    }

    /// Creates a new binding in the closest state.
    ///
    pub fn insert_binding(&mut self, name: &str, binding: Binding) -> &mut Self {
        self.enforce_non_empty_stack();

        self.stack
            .last_mut()
            .unwrap()
            .bindings
            .insert(name.to_string(), binding);
        self
    }

    /// Pushes a new state without any mark.
    ///
    pub fn push(&mut self, mode: Mode) -> &mut Self {
        self.push_internal(Mark::None, mode)
    }

    /// Pushes a new state with a call mark.
    ///
    pub fn push_call(&mut self, mode: Mode) -> &mut Self {
        self.push_internal(Mark::Call, mode)
    }

    /// Pushes a new state with a loop mark.
    ///
    pub fn push_loop(&mut self, mode: Mode) -> &mut Self {
        self.push_internal(Mark::Loop, mode)
    }

    /// Pops the last state.
    ///
    pub fn pop(&mut self) -> &mut Self {
        self.enforce_non_empty_stack();

        self.stack.pop();
        self
    }

    /// Pushes a new state.
    ///
    fn push_internal(&mut self, mark: Mark, mode: Mode) -> &mut Self {
        self.stack.push(State {
            mark,
            mode,
            bindings: HashMap::new(),
        });
        self
    }

    /// Panics if the stack is empty.
    ///
    fn enforce_non_empty_stack(&self) {
        if self.is_empty() {
            panic!("the evaluator has no state !");
        }
    }
}
