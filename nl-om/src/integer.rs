use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new boolean object.
    ///
    pub fn integer(v: i32) -> Self {
        Self::Integer(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is an integer.
    ///
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_, _) => true,
            _ => false,
        }
    }

    /// Gets the integer value of the object.
    ///
    pub fn as_integer(&self) -> Option<i32> {
        match self {
            Self::Integer(_, v) => Some(*v),
            _ => None,
        }
    }

    /// Gets the integer value of the object.
    ///
    pub fn as_integer_mut(&mut self) -> Option<&mut i32> {
        match self {
            Self::Integer(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the integer value of the object.
    /// Panics if the object isn't an integer.
    ///
    pub fn get_integer(&self) -> i32 {
        self.as_integer().expect("Expected an Integer.")
    }

    /// Gets the integer value of the object.
    /// Panics if the object isn't an integer.
    ///
    pub fn get_integer_mut(&mut self) -> &mut i32 {
        self.as_integer_mut().expect("Expected an :Integer.")
    }
}
