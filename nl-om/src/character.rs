use super::*;

use std::char;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new char object.
    ///
    pub fn char(v: char) -> Self {
        Self::Char(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is a char.
    ///
    pub fn is_char(&self) -> bool {
        match self {
            Self::Char(_, _) => true,
            _ => false,
        }
    }

    /// Gets the char in the object.
    ///
    pub fn as_char(&self) -> Option<char> {
        match self {
            Self::Char(_, v) => Some(*v),
            _ => None,
        }
    }

    /// Gets the char in the object.
    ///
    pub fn as_char_mut(&mut self) -> Option<&mut char> {
        match self {
            Self::Char(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the char in the object.
    /// Panics if the object isn't a char.
    ///
    pub fn get_char(&self) -> char {
        self.as_char().expect("Expected an Object::Char")
    }

    /// Gets the char in the object.
    /// Panics if the object isn't a char.
    ///
    pub fn get_char_mut(&mut self) -> &mut char {
        self.as_char_mut().expect("Expected an Object::Char")
    }
}
