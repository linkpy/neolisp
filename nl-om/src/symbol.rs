use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new symbol object.
    ///
    pub fn symbol(v: String) -> Self {
        Self::Symbol(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is a symbol.
    ///
    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Symbol(_, _) => true,
            _ => false,
        }
    }

    /// Gets the symbol in the object.
    ///
    pub fn as_symbol(&self) -> Option<&String> {
        match self {
            Self::Symbol(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the symbol in the object.
    ///
    pub fn as_symbol_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Symbol(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the symbol in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_symbol(&self) -> &String {
        self.as_symbol().expect("Expected a Symbol")
    }

    /// Gets the symbol in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_symbol_mut(&mut self) -> &mut String {
        self.as_symbol_mut().expect("Expected a Symbol")
    }
}
