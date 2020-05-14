use std::fmt;

/// Direct location, used when the info comes from
/// a source file directly.
///
#[derive(Clone)]
pub struct DirectLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,

    pub index: usize,
    pub length: usize,
}

impl DirectLocation {
    /// Creates a new incomplete direct location.
    ///
    pub fn new(from: usize, to: usize, file: &str, source: &str) -> DirectLocation {
        let before = &source[0..from];

        let (last_newline, line) = before
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n')
            .fold((0usize, 0usize), |(_, count), (idx, _)| (idx, count + 1));

        DirectLocation {
            file: file.to_string(),
            line: line + 1,
            column: 1 + (from - last_newline),
            index: from,
            length: to - from,
        }
    }
}

/// Expansion location, used when the info comes from
/// a macro expansion.
///
#[derive(Clone)]
pub struct ExpansionLocation {
    pub macro_name: String,
    pub form: String,

    pub location: DirectLocation,
}

/// A location.
///
#[derive(Clone)]
pub enum Location {
    /// No location information.
    None,
    /// Direct location.
    Direct(DirectLocation),
    /// Expansion location.
    Expansion(Vec<ExpansionLocation>),
}

//==================================================================================================
// Implementation

impl Location {
    /// Creates a new empty object location.
    ///
    pub fn new_empty() -> Location {
        Location::None
    }

    /// Creates a new direct object location.
    ///
    pub fn new(f: usize, t: usize, file: &str, source: &str) -> Location {
        Location::Direct(DirectLocation::new(f, t, file, source))
    }

    /// Checks if there is no location info.
    ///
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    /// Checks if the location info comes from a source file.
    ///
    pub fn is_direct(&self) -> bool {
        match self {
            Self::Direct(_) => true,
            _ => false,
        }
    }

    /// Checks if the location info comes from a macro expansion.
    ///
    pub fn is_expansion(&self) -> bool {
        match self {
            Self::Expansion(_) => true,
            _ => false,
        }
    }

    /// Gets the location as direct.
    ///
    pub fn as_direct(&self) -> Option<&DirectLocation> {
        match self {
            Self::Direct(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the location as expansion.
    ///
    pub fn as_expansion(&self) -> Option<&Vec<ExpansionLocation>> {
        match self {
            Self::Expansion(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the location as direct.
    ///
    pub fn as_direct_mut(&mut self) -> Option<&mut DirectLocation> {
        match self {
            Self::Direct(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the location as expansion.
    ///
    pub fn as_expansion_mut(&mut self) -> Option<&mut Vec<ExpansionLocation>> {
        match self {
            Self::Expansion(v) => Some(v),
            _ => None,
        }
    }
    /// Gets the location as direct.
    ///
    pub fn get_direct(&self) -> &DirectLocation {
        self.as_direct().expect("Expected a direct localisation.")
    }

    /// Gets the location as expansion.
    ///
    pub fn get_expansion(&self) -> &Vec<ExpansionLocation> {
        self.as_expansion()
            .expect("Expected an expansion localisation.")
    }

    /// Gets the location as direct.
    ///
    pub fn get_direct_mut(&mut self) -> &mut DirectLocation {
        self.as_direct_mut()
            .expect("Expected a direct localisation.")
    }

    /// Gets the location as expansion.
    ///
    pub fn get_expansion_mut(&mut self) -> &mut Vec<ExpansionLocation> {
        self.as_expansion_mut()
            .expect("Expected an expansion localisation.")
    }
}

//==================================================================================================
// Trait implementations

impl fmt::Display for DirectLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "in file '{}', at {}:{}",
            self.file, self.line, self.column
        )
    }
}

impl fmt::Display for ExpansionLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "from the expansion of '{}', {}, from the expression {}",
            self.macro_name, self.location, self.form
        )
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "no location"),
            Self::Direct(v) => write!(f, "{}", v),
            Self::Expansion(v) => {
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}", x)?;

                    if i + 1 < v.len() {
                        write!(f, "\n")?;
                    }
                }

                Ok(())
            }
        }
    }
}
