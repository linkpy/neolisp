use super::*;

use std::char;

impl Object {
    /// Creates a new char object.
    ///
    pub fn char(v: char) -> Self {
        Self::Char(ObjectInfo::new_empty(), v)
    }

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

    /// Converts the object to a char.
    ///
    pub fn to_char(&self) -> char {
        match self {
            Self::Nil(_) => '\0',
            Self::Bool(_, v) => {
                if *v {
                    1 as char
                } else {
                    '\0'
                }
            }
            Self::Integer(_, v) => char::from_u32(*v as u32).unwrap_or('\0'),
            Self::Float(_, v) => char::from_u32(*v as u32).unwrap_or('\0'),
            Self::Char(_, v) => *v,
            Self::String(_, v) => {
                if v.len() > 0 {
                    v.chars().next().unwrap()
                } else {
                    '\0'
                }
            }
            Self::Keyword(_, v) => {
                if v.len() > 0 {
                    v.chars().next().unwrap()
                } else {
                    '\0'
                }
            }
            Self::Symbol(_, v) => {
                if v.len() > 0 {
                    v.chars().next().unwrap()
                } else {
                    '\0'
                }
            }
            Self::List(_, v) => {
                if v.len() > 0 {
                    1 as char
                } else {
                    '\0'
                }
            }
        }
    }

    /// Converts the object into a char object.
    ///
    pub fn to_char_obj(&self) -> Self {
        Self::char(self.to_char())
    }

    /// Gets the char in the object.
    /// Panics if the object isn't a char.
    ///
    pub fn get_char(&self) -> char {
        self.as_char().expect("Expected an Object::Char")
    }

    /// Sets the object to be a char.
    ///
    pub fn set_char(&mut self, v: char) -> &mut Self {
        *self = Self::char(v);
        self
    }
}
