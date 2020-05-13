use super::*;

impl Object {
    /// Creates a new symbol object.
    ///
    pub fn symbol(v: String) -> Self {
        Self::Symbol(ObjectInfo::new_empty(), v)
    }

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

    /// Converts the object into a symbol.
    ///
    pub fn to_symbol(&self) -> String {
        match self {
            Self::Nil(_) => "nil".to_string(),
            Self::Bool(_, v) => {
                if *v {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Self::Integer(_, v) => v.to_string(),
            Self::Float(_, v) => v.to_string(),
            Self::Char(_, v) => v.to_string(),
            Self::String(_, v) => v.clone(),
            Self::Keyword(_, v) => v.clone(),
            Self::Symbol(_, v) => v.clone(),
            Self::List(_, v) => {
                let mut r = "(".to_string();
                for i in v {
                    r.push_str(&i.to_string());
                    r.push(' ');
                }
                r.push(')');
                r
            }
        }
    }

    /// Converts the object into a symbol object.
    ///
    pub fn to_symbol_obj(&self) -> Object {
        Self::symbol(self.to_symbol())
    }

    /// Gets the symbol in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_symbol(&self) -> &String {
        self.as_symbol().expect("Expected an Object::Symbol")
    }

    /// Gets the symbol in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_symbol_mut(&mut self) -> &mut String {
        self.as_symbol_mut().expect("Expected an Object::Symbol")
    }

    /// Sets the object to be a symbol.
    ///
    pub fn set_symbol(&mut self, v: String) -> &mut Self {
        *self = Self::symbol(v);
        self
    }
}
