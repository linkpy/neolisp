use super::*;

impl Object {
    /// Creates a new boolean object.
    ///
    pub fn bool(v: bool) -> Self {
        Self::Bool(ObjectInfo::new_empty(), v)
    }

    /// Checks if the object is a boolean.
    ///
    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_, _) => true,
            _ => false,
        }
    }

    /// Gets the boolean value of the object.
    ///
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(_, v) => Some(*v),
            _ => None,
        }
    }

    /// Converts the object into a boolean.
    ///
    pub fn to_bool(&self) -> bool {
        match self {
            Self::Nil(_) => false,
            Self::Bool(_, v) => *v,
            Self::Integer(_, v) => *v != 0,
            Self::Float(_, v) => *v != 0.0,
            Self::Char(_, v) => *v != '\0',
            Self::String(_, v) => !v.is_empty(),
            Self::Keyword(_, _) => true,
            Self::Symbol(_, _) => true,
            Self::List(_, v) => !v.is_empty(),
        }
    }

    /// Converts the object into a boolean object.
    ///
    pub fn to_bool_obj(&self) -> Self {
        Self::bool(self.to_bool())
    }

    /// Gets the boolean value of the object.
    /// Panics if the object isn't a boolean.
    ///
    pub fn get_bool(&self) -> bool {
        self.as_bool().expect("Expected an Object::Bool.")
    }

    /// Sets the object to be a boolean.
    ///
    pub fn set_bool(&mut self, v: bool) -> &mut Self {
        *self = Self::bool(v);
        self
    }
}
