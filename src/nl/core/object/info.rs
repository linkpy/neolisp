pub use crate::nl::core::location::*;

/// Extra information attached to an object.
///
#[derive(Clone)]
pub struct ObjectInfo {
    /// Localisation where the object was initially created.
    pub location: Location,
}

impl ObjectInfo {
    /// Creates a new object information record.
    ///
    pub fn new(loc: Location) -> Self {
        ObjectInfo { location: loc }
    }

    /// Creates a new empty object information record.
    ///
    pub fn new_empty() -> Self {
        Self::new(Location::None)
    }
}
