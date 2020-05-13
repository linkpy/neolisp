use super::*;

impl Object {
    /// Creates a new nil object.
    ///
    pub fn nil() -> Self {
        Self::Nil(ObjectInfo::new_empty())
    }

    /// Returns true if the object is nil.
    ///
    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil(_) => true,
            _ => false,
        }
    }

    /// Sets the object to be nil.
    ///
    pub fn set_nil(&mut self) -> &mut Self {
        *self = Object::nil();
        self
    }
}
