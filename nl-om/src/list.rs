use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new list object.
    ///
    pub fn list(v: Vec<Self>) -> Self {
        Self::List(T::default(), v)
    }
}

impl<T> Object<T> {
    /// Checks if the object is a list.
    ///
    pub fn is_list(&self) -> bool {
        match self {
            Self::List(_, _) => true,
            _ => false,
        }
    }

    /// Gets the list of the object.
    ///
    pub fn as_list(&self) -> Option<&Vec<Self>> {
        match self {
            Self::List(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the list of the object.
    ///
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::List(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the list of the object.
    /// Panics if the object isn't a list.
    ///
    pub fn get_list(&self) -> &Vec<Self> {
        self.as_list().expect("Expected a List.")
    }

    /// Gets the list of the object.
    /// Panics if the object isn't a list.
    ///
    pub fn get_list_mut(&mut self) -> &mut Vec<Self> {
        self.as_list_mut().expect("Expected a List.")
    }
}
