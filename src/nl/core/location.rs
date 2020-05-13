use crate::nl::core::object::*;

use std::fmt;
use std::fmt::Display;

/// Intern location, used when the info comes from
/// NeoLisp itself.
///
#[derive(Clone)]
pub struct InternLocation {
    /// File in which the info comes form.
    pub file: String,
    /// Line in the file.
    pub line: u32,
}

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
    pub fn new_light(from: usize, to: usize) -> DirectLocation {
        DirectLocation {
            file: "".to_string(),
            line: 0,
            column: 0,
            index: from,
            length: to - from,
        }
    }

    /// Completes the direct location.
    ///
    pub fn complete(&mut self, file: &String, source: &String) {
        let before = &source[0..self.index];

        let (last_newline, line) = before
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n')
            .fold((0usize, 0usize), |(_, count), (idx, _)| (idx, count + 1));

        self.file = file.clone();
        self.line = 1 + line;
        self.column = 1 + (self.index - last_newline);
    }
}

/// Expansion location, used when the info comes from
/// a macro expansion.
///
#[derive(Clone)]
pub struct ExpansionLocation {
    pub macro_name: String,
    pub form: Object,

    pub localisation: DirectLocation,
}

/// A location.
///
#[derive(Clone)]
pub enum Location {
    /// No location information.
    None,
    /// Intern location.
    Intern(InternLocation),
    /// Direct location.
    Direct(DirectLocation),
    /// Expansion location.
    Expansion(Vec<ExpansionLocation>),
}

#[macro_export]
macro_rules! intern_location {
    () => {{
        Location::new_intern(file!(), line!())
    }};
}

impl Location {
    /// Creates a new empty object location.
    ///
    pub fn new_empty() -> Location {
        Location::None
    }

    /// Creates a new intern object location.
    ///
    pub fn new_intern(file: &str, line: u32) -> Location {
        Location::Intern(InternLocation {
            file: file.to_string(),
            line,
        })
    }

    /// Creates a new direct object location.
    ///
    pub fn new_direct(f: usize, t: usize) -> Location {
        Location::Direct(DirectLocation::new_light(f, t))
    }

    /// Checks if there is no location info.
    ///
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    /// Checks if the location info comes from NeoLisp itself.
    ///
    pub fn is_intern(&self) -> bool {
        match self {
            Self::Intern(_) => true,
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

    /// Gets the location as intern.
    ///
    pub fn as_intern(&self) -> Option<&InternLocation> {
        match self {
            Self::Intern(v) => Some(v),
            _ => None,
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

    /// Gets the location as intern.
    ///
    pub fn as_intern_mut(&mut self) -> Option<&mut InternLocation> {
        match self {
            Self::Intern(v) => Some(v),
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

    /// Gets the location as intern.
    ///
    pub fn get_intern(&self) -> &InternLocation {
        self.as_intern().expect("Expected an intern localisation.")
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

    /// Gets the location as intern.
    ///
    pub fn get_intern_mut(&mut self) -> &mut InternLocation {
        self.as_intern_mut()
            .expect("Expected an intern localisation.")
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

impl Display for InternLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "in '{}' line {}", self.file, self.line)
    }
}

impl Display for DirectLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "in file '{}', at {}:{}",
            self.file, self.line, self.column
        )
    }
}

impl Display for ExpansionLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "from the expansion of '{}', {}, from the expression {}",
            self.macro_name, self.localisation, self.form
        )
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "no location"),
            Self::Intern(v) => write!(f, "{}", v),
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
