use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new nil object with the default information.
    ///
    pub fn nil() -> Self {
        Self::Nil(T::default())
    }
}

impl<T> Object<T> {
    /// Returns true if the object is nil.
    ///
    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil(_) => true,
            _ => false,
        }
    }
}
