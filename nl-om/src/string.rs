use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new string object.
    ///
    pub fn string(v: String) -> Self {
        Self::String(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is a string.
    ///
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_, _) => true,
            _ => false,
        }
    }

    /// Gets the string in the object.
    ///
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the string in the object.
    ///
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::String(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the string in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_string(&self) -> &String {
        self.as_string().expect("Expected a String")
    }

    /// Gets the string in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_string_mut(&mut self) -> &mut String {
        self.as_string_mut().expect("Expected a String")
    }
}
