use super::*;

impl Object {
    /// Creates a new keyword object.
    ///
    pub fn keyword(v: String) -> Self {
        Self::Keyword(ObjectInfo::new_empty(), v)
    }

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

    /// Converts the object into a keyword.
    ///
    pub fn to_keyword(&self) -> String {
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

    /// Converts the object into a keyword object.
    ///
    pub fn to_keyword_obj(&self) -> Object {
        Self::keyword(self.to_keyword())
    }

    /// Gets the keyword in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_keyword(&self) -> &String {
        self.as_keyword().expect("Expected an Object::Keyword")
    }

    /// Gets the keyword in the object.
    /// Panics if the object isn't a string.
    ///
    pub fn get_keyword_mut(&mut self) -> &mut String {
        self.as_keyword_mut().expect("Expected an Object::Keyword")
    }

    /// Sets the object to be a keyword.
    ///
    pub fn set_keyword(&mut self, v: String) -> &mut Self {
        *self = Self::keyword(v);
        self
    }
}
