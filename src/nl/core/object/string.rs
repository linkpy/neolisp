use super::*;

impl Object {
    /// Creates a new string object.
    ///
    pub fn string(v: String) -> Self {
        Self::String(ObjectInfo::new_empty(), v)
    }

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

    /// Converts the object into a string.
    ///
    pub fn to_string(&self) -> String {
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

    /// Converts the object into a string object.
    ///
    pub fn to_string_obj(&self) -> Object {
        Self::string(self.to_string())
    }

    /// Gets the string in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_string(&self) -> &String {
        self.as_string().expect("Expected an Object::String")
    }

    /// Gets the string in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_string_mut(&mut self) -> &mut String {
        self.as_string_mut().expect("Expected an Object::String")
    }

    /// Sets the object to be a string.
    ///
    pub fn set_string(&mut self, v: String) -> &mut Self {
        *self = Self::string(v);
        self
    }
}
