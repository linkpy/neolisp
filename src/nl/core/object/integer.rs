use super::*;

impl Object {
    /// Creates a new boolean object.
    ///
    pub fn integer(v: i32) -> Self {
        Self::Integer(ObjectInfo::new_empty(), v)
    }

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

    /// Converts the object into an integer.
    ///
    pub fn to_integer(&self) -> i32 {
        match self {
            Self::Nil(_) => 0,
            Self::Bool(_, v) => {
                if *v {
                    1
                } else {
                    0
                }
            }
            Self::Integer(_, v) => *v,
            Self::Float(_, v) => *v as i32,
            Self::Char(_, v) => *v as i32,
            Self::String(_, v) => v.len() as i32,
            Self::Keyword(_, v) => v.len() as i32,
            Self::Symbol(_, v) => v.len() as i32,
            Self::List(_, v) => v.len() as i32,
        }
    }

    /// Converts the object into an integer object.
    ///
    pub fn to_integer_obj(&self) -> Self {
        Self::integer(self.to_integer())
    }

    /// Gets the integer value of the object.
    /// Panics if the object isn't an integer.
    ///
    pub fn get_integer(&self) -> i32 {
        self.as_integer().expect("Expected an Object::Integer.")
    }

    /// Sets the object to be an integer.
    ///
    pub fn set_integer(&mut self, v: i32) -> &mut Self {
        *self = Self::integer(v);
        self
    }
}
