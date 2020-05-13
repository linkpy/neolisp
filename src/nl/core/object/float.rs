use super::*;

impl Object {
    /// Creates a new float object.
    ///
    pub fn float(v: f32) -> Self {
        Self::Float(ObjectInfo::new_empty(), v)
    }

    /// Checks if the object is a float.
    ///
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_, _) => true,
            _ => false,
        }
    }

    /// Gets the float of the object.
    ///
    pub fn as_float(&self) -> Option<f32> {
        match self {
            Self::Float(_, v) => Some(*v),
            _ => None,
        }
    }

    /// Converts the object into a float.
    ///
    pub fn to_float(&self) -> f32 {
        match self {
            Self::Nil(_) => 0.0,
            Self::Bool(_, v) => {
                if *v {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Integer(_, v) => *v as f32,
            Self::Float(_, v) => *v,
            Self::Char(_, v) => (*v as u32) as f32,
            Self::String(_, v) => v.len() as f32,
            Self::Keyword(_, v) => v.len() as f32,
            Self::Symbol(_, v) => v.len() as f32,
            Self::List(_, v) => v.len() as f32,
        }
    }

    /// Converts the object into a float object.
    ///
    pub fn to_float_obj(&self) -> Self {
        Self::float(self.to_float())
    }

    /// Gets the float of the object.
    /// Panics if the object isn't a float.
    ///
    pub fn get_float(&self) -> f32 {
        self.as_float().expect("Expected an Object::Float.")
    }

    /// Sets the object to be a float.
    ///
    pub fn set_float(&mut self, v: f32) -> &mut Self {
        *self = Self::float(v);
        self
    }
}
