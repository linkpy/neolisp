use super::*;

impl Object {
    /// Creates a new list object.
    ///
    pub fn list(v: Vec<Object>) -> Self {
        Self::List(ObjectInfo::new_empty(), v)
    }

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
    pub fn as_list(&self) -> Option<&Vec<Object>> {
        match self {
            Self::List(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the list of the object.
    ///
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Object>> {
        match self {
            Self::List(_, v) => Some(v),
            _ => None,
        }
    }

    /// Converts the object into a list.
    ///
    pub fn to_list(&self) -> Vec<Self> {
        match self {
            Self::Nil(_) => Vec::new(),
            Self::Bool(_, v) => vec![Self::bool(*v)],
            Self::Integer(_, v) => vec![Self::integer(*v)],
            Self::Float(_, v) => vec![Self::float(*v)],
            Self::Char(_, v) => vec![Self::char(*v)],
            Self::String(_, v) => vec![Self::string(v.clone())],
            Self::Keyword(_, v) => vec![Self::keyword(v.clone())],
            Self::Symbol(_, v) => vec![Self::symbol(v.clone())],
            Self::List(_, v) => v.clone(),
        }
    }

    /// Converts the object into a list object.
    ///
    pub fn to_list_obj(&self) -> Self {
        Self::list(self.to_list())
    }

    /// Gets the list of the object.
    /// Panics if the object isn't a list.
    ///
    pub fn get_list(&self) -> &Vec<Self> {
        self.as_list().expect("Expected an Object::List.")
    }

    /// Gets the list of the object.
    /// Panics if the object isn't a list.
    ///
    pub fn get_list_mut(&mut self) -> &mut Vec<Self> {
        self.as_list_mut().expect("Expected an Object::List.")
    }

    /// Sets the object to be a list.
    ///
    pub fn set_list(&mut self, v: Vec<Self>) -> &mut Self {
        *self = Self::list(v);
        self
    }
}
