use super::*;

impl<T> Object<T>
where
    T: Default,
{
    /// Creates a new float object.
    ///
    pub fn float(v: f32) -> Self {
        Self::Float(T::default(), v)
    }
}

impl<T> Object<T> {
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

    /// Gets the float of the object.
    ///
    pub fn as_float_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Float(_, v) => Some(v),
            _ => None,
        }
    }

    /// Gets the float of the object.
    /// Panics if the object isn't a float.
    ///
    pub fn get_float(&self) -> f32 {
        self.as_float().expect("Expected an Float.")
    }

    /// Gets the float of the object.
    /// Panics if the object isn't a float.
    ///
    pub fn get_float_mut(&mut self) -> &mut f32 {
        self.as_float_mut().expect("Expected an Float.")
    }
}
