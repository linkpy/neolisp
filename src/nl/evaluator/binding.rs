use super::*;

/// A scope binding.
///
pub enum Binding {
    /// Holds a constant.
    Constant(Object),
    /// Holds a variable.
    Variable(Object),

    /// Holds a form.
    Form(Form),
}

/// A form understood by the evaluation engine.
///
pub enum Form {
    /// A special form. Its arguments aren't evaluated, and can modify the engine.
    Special(SpecialForm),
    /// An interface form. Its arguments are evaluated, and can't access the engine.
    Interface(InterfaceForm),
    /// A function defined during evaluation. Its arguments are evaluated.
    Dynamic(LispForm),
    /// A function defined during evaluation. Its arguments aren't evaluated, and based
    /// on the engine's mode, its return value might be evaluated.
    Macro(LispForm),
}

/// Aliases for the special native forms.
pub type SpecialForm = NativeForm<SpecialFormHandler>;
/// Aliases for the interface native forms.
pub type InterfaceForm = NativeForm<InterfaceFormHandler>;

/// A native form description record.
///
pub struct NativeForm<T> {
    /// Native handler of the form.
    handler: T,
    /// Number of argument accepted by the form.
    argument_count: i32,
    /// The kinds of the accepted arguments.
    argument_kinds: Vec<ObjectKind>,
}

/// Native handler for a special form.
pub type SpecialFormHandler = fn(&mut Evaluator, &[Object]) -> Result<Object, Error>;
/// Native handler for an interface form.
pub type InterfaceFormHandler = fn(&[Object]) -> Result<Object, Error>;

/// A Lisp form description record. Used for functions defined during evaluations.
///
pub struct LispForm {
    /// Location where the form was defined.
    location: Location,
    /// Number of argument accepted by the form.
    argument_count: i32,
    /// Names of the argument of the form.
    argument_names: Vec<String>,
    /// Expressions forming the body of the form.
    expressions: Vec<Object>,
}

//==================================================================================================
// Implementations

impl Binding {
    /// Checks if the binding is a constant.
    ///
    pub fn is_constant(&self) -> bool {
        match self {
            Self::Constant(_) => true,
            _ => false,
        }
    }

    /// Checks if the binding is a variable.
    ///
    pub fn is_variable(&self) -> bool {
        match self {
            Self::Variable(_) => true,
            _ => false,
        }
    }

    /// Checks if the binding is a form.
    ///
    pub fn is_form(&self) -> bool {
        match self {
            Self::Form(_) => true,
            _ => false,
        }
    }

    /// Gets the constant's value from the binding.
    ///
    pub fn as_constant(&self) -> Option<&Object> {
        match self {
            Self::Constant(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the constant's value from the binding.
    ///
    pub fn as_constant_mut(&mut self) -> Option<&mut Object> {
        match self {
            Self::Constant(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the variable's value from the binding.
    ///
    pub fn as_variable(&self) -> Option<&Object> {
        match self {
            Self::Variable(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the variable's value from the binding.
    ///
    pub fn as_variable_mut(&mut self) -> Option<&mut Object> {
        match self {
            Self::Variable(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form's value from the binding.
    ///
    pub fn as_form(&self) -> Option<&Form> {
        match self {
            Self::Form(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form's value from the binding.
    ///
    pub fn as_form_mut(&mut self) -> Option<&mut Form> {
        match self {
            Self::Form(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the constant's value from the binding.
    ///
    pub fn get_constant(&self) -> &Object {
        self.as_constant().expect("expected a constant binding")
    }

    /// Gets the constant's value from the binding.
    ///
    pub fn get_constant_mut(&mut self) -> &mut Object {
        self.as_constant_mut().expect("expected a constant binding")
    }

    /// Gets the variable's value from the binding.
    ///
    pub fn get_variable(&self) -> &Object {
        self.as_variable().expect("expected a variable binding")
    }

    /// Gets the variable's value from the binding.
    ///
    pub fn get_variable_mut(&mut self) -> &mut Object {
        self.as_variable_mut().expect("expected a variable binding")
    }

    /// Gets the form's value from the binding.
    ///
    pub fn get_form(&self) -> &Form {
        self.as_form().expect("expected a constant binding")
    }

    /// Gets the form's value from the binding.
    ///
    pub fn get_form_mut(&mut self) -> &mut Form {
        self.as_form_mut().expect("expected a constant binding")
    }
}

impl Form {
    /// Checks if the form is a special.
    ///
    pub fn is_special(&self) -> bool {
        match self {
            Self::Special(_) => true,
            _ => false,
        }
    }

    /// Checks if the form is an interface.
    ///
    pub fn is_interface(&self) -> bool {
        match self {
            Self::Interface(_) => true,
            _ => false,
        }
    }

    /// Checks if the form is a dynamic.
    ///
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic(_) => true,
            _ => false,
        }
    }

    /// Checks if the form is a macro.
    ///
    pub fn is_macro(&self) -> bool {
        match self {
            Self::Macro(_) => true,
            _ => false,
        }
    }

    /// Gets the form as a special.
    ///
    pub fn as_special(&self) -> Option<&SpecialForm> {
        match self {
            Self::Special(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a special.
    ///
    pub fn as_special_mut(&mut self) -> Option<&mut SpecialForm> {
        match self {
            Self::Special(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as an interface.
    ///
    pub fn as_interface(&self) -> Option<&InterfaceForm> {
        match self {
            Self::Interface(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as an interface.
    ///
    pub fn as_interface_mut(&mut self) -> Option<&mut InterfaceForm> {
        match self {
            Self::Interface(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a dynamic.
    ///
    pub fn as_dynamic(&self) -> Option<&LispForm> {
        match self {
            Self::Dynamic(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a dynamic.
    ///
    pub fn as_dynamic_mut(&mut self) -> Option<&mut LispForm> {
        match self {
            Self::Dynamic(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a macro.
    ///
    pub fn as_macro(&self) -> Option<&LispForm> {
        match self {
            Self::Macro(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a macro.
    ///
    pub fn as_macro_mut(&mut self) -> Option<&mut LispForm> {
        match self {
            Self::Dynamic(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the form as a special.
    ///
    pub fn get_special(&self) -> &SpecialForm {
        self.as_special().expect("expected a special form")
    }

    /// Gets the form as a special.
    ///
    pub fn get_special_mut(&mut self) -> &mut SpecialForm {
        self.as_special_mut().expect("expected a special form")
    }

    /// Gets the form as an interface.
    ///
    pub fn get_interface(&self) -> &InterfaceForm {
        self.as_interface().expect("expected a interface form")
    }

    /// Gets the form as a interface.
    ///
    pub fn get_interface_mut(&mut self) -> &mut InterfaceForm {
        self.as_interface_mut().expect("expected a interface form")
    }

    /// Gets the form as a dynamic.
    ///
    pub fn get_dynamic(&self) -> &LispForm {
        self.as_dynamic().expect("expected a dynamic form")
    }

    /// Gets the form as a dynamic.
    ///
    pub fn get_dynamic_mut(&mut self) -> &mut LispForm {
        self.as_dynamic_mut().expect("expected a dynamic form")
    }

    /// Gets the form as a macro.
    ///
    pub fn get_macro(&self) -> &LispForm {
        self.as_macro().expect("expected a macro form")
    }

    /// Gets the form as a macro.
    ///
    pub fn get_macro_mut(&mut self) -> &mut LispForm {
        self.as_macro_mut().expect("expected a macro form")
    }
}

impl<T> NativeForm<T> {
    /// Creates a new native form.
    ///
    /// The argument count follows the given behaviors :
    ///
    ///  - If >= 0, then the form takes exactly N arguments.
    ///    In that case, N kinds must be provided.
    ///  - If < 0, then the form takes at least N-1 arguments (-1 = 0+, -2 = 1+, ...).
    ///    In that case, N+1 kinds must be provided. The extra kind is used for the extra argument.
    ///    The function will then receive N+1 arguments, with the extra one being a list.
    ///
    pub fn new(handler: T, argument_count: i32, argument_kinds: Vec<ObjectKind>) -> Self {
        Self {
            handler,
            argument_count,
            argument_kinds,
        }
    }

    /// Gets the handler of the form.
    ///
    pub fn handler(&self) -> &T {
        &self.handler
    }

    /// Gets the argument count of the form.
    ///
    pub fn argument_count(&self) -> i32 {
        self.argument_count
    }

    /// Gets the argument kinds accepted by the form.
    ///
    pub fn argument_kinds(&self) -> &Vec<ObjectKind> {
        &self.argument_kinds
    }
}

impl LispForm {
    /// Creates a new lisp form.
    ///
    /// The argument count follows the given behaviors :
    ///
    ///  - If >= 0, then the form takes exactly N arguments.
    ///    In that case, N kinds and names must be provided.
    ///  - If < 0, then the form takes at least N-1 arguments (-1 = 0+, -2 = 1+, ...).
    ///    In that case, N+1 kinds and names must be provided. The extra kind is used for the extra
    ///    argument and the extra name is used for the extra argument list binding's name.
    ///    The function will then receive N+1 arguments, with the extra one being a list.
    ///
    pub fn new(
        location: Location,
        argument_count: i32,
        argument_names: Vec<String>,
        expressions: Vec<Object>,
    ) -> LispForm {
        LispForm {
            location,
            argument_count,
            argument_names,
            expressions,
        }
    }

    /// Gets the location where the form was defined.
    ///
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Gets the number of argument the form receives.
    ///
    pub fn argument_count(&self) -> i32 {
        self.argument_count
    }

    /// Gets the names of the form's arguments.
    ///
    pub fn argument_names(&self) -> &Vec<String> {
        &self.argument_names
    }

    /// Gets the expressions making the form's body.
    ///
    pub fn expressions(&self) -> &Vec<Object> {
        &self.expressions
    }
}
