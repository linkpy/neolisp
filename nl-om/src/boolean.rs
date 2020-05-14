use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new boolean object.
    ///
    pub fn bool(v: bool) -> Self {
        Self::Bool(T::default(), v)
    }
}

impl<T> Object<T> {
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

    /// Gets the boolean value of the object.
    ///
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Bool(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the boolean value of the object.
    /// Panics if the object isn't a boolean.
    ///
    pub fn get_bool(&self) -> bool {
        self.as_bool().expect("Expected a Bool.")
    }

    /// Gets the boolean value of the object.
    /// Panics if the object isn't a boolean.
    ///
    pub fn get_bool_mut(&mut self) -> &mut bool {
        self.as_bool_mut().expect("Expected a Bool.")
    }
}
