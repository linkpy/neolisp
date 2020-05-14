use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new keyword object.
    ///
    pub fn keyword(v: String) -> Self {
        Self::Keyword(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is a keyword.
    ///
    pub fn is_keyword(&self) -> bool {
        match self {
            Self::Keyword(_, _) => true,
            _ => false,
        }
    }

    /// Gets the keyword in the object.
    ///
    pub fn as_keyword(&self) -> Option<&String> {
        match self {
            Self::Keyword(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the keyword in the object.
    ///
    pub fn as_keyword_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Keyword(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the keyword in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_keyword(&self) -> &String {
        self.as_keyword().expect("Expected a Keyword")
    }

    /// Gets the keyword in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_keyword_mut(&mut self) -> &mut String {
        self.as_keyword_mut().expect("Expected a Keyword")
    }
}
