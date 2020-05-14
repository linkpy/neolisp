mod boolean;
mod character;
mod float;
mod integer;
mod keyword;
mod list;
mod nil;
mod string;
mod symbol;

pub use boolean::*;
pub use character::*;
pub use float::*;
pub use integer::*;
pub use keyword::*;
pub use list::*;
pub use nil::*;
pub use string::*;
pub use symbol::*;

use colored::Colorize;
use std::fmt;

/// The kind of an object.
///
#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    /// A null object.
    Nil,
    /// A boolean object.
    Bool,
    /// An integer object.
    Integer,
    /// A float object.
    Float,
    /// A char object.
    Char,
    /// A string object.
    String,
    /// A keyword object.
    Keyword,
    /// A symbol object.
    Symbol,
    /// A list object.
    List,

    /// An object of any kind.
    Any,
}

/// A NeoLisp object.
///
/// It accepts a generic type which will be used as meta information. These info
/// aren't used by the Object Model, and are for implementations that need extra info
/// to be associated to any given object.
///
#[derive(Clone)]
pub enum Object<T> {
    /// A null object.
    Nil(T),
    /// A boolean object.
    Bool(T, bool),
    /// An integer object.
    Integer(T, i32),
    /// A float object.
    Float(T, f32),
    /// A character object.
    Char(T, char),
    /// A string object.
    String(T, String),
    /// A keyword object.
    Keyword(T, String),
    /// A symbol object.
    Symbol(T, String),
    /// A list object.
    List(T, Vec<Self>),
}

//==================================================================================================
// Implementations

impl<T> Object<T> {
    /// Gets the kind of the object.
    ///
    pub fn kind(&self) -> Kind {
        match self {
            Object::Nil(_) => Kind::Nil,
            Object::Bool(_, _) => Kind::Bool,
            Object::Integer(_, _) => Kind::Integer,
            Object::Float(_, _) => Kind::Float,
            Object::Char(_, _) => Kind::Char,
            Object::String(_, _) => Kind::String,
            Object::Keyword(_, _) => Kind::Keyword,
            Object::Symbol(_, _) => Kind::Symbol,
            Object::List(_, _) => Kind::List,
        }
    }

    /// Gets the information associated with the object.
    ///
    pub fn info(&self) -> &T {
        match self {
            Object::Nil(i) => i,
            Object::Bool(i, _) => i,
            Object::Integer(i, _) => i,
            Object::Float(i, _) => i,
            Object::Char(i, _) => i,
            Object::String(i, _) => i,
            Object::Keyword(i, _) => i,
            Object::Symbol(i, _) => i,
            Object::List(i, _) => i,
        }
    }

    /// Gets the information associated with the object.
    ///
    pub fn info_mut(&mut self) -> &T {
        match self {
            Object::Nil(i) => i,
            Object::Bool(i, _) => i,
            Object::Integer(i, _) => i,
            Object::Float(i, _) => i,
            Object::Char(i, _) => i,
            Object::String(i, _) => i,
            Object::Keyword(i, _) => i,
            Object::Symbol(i, _) => i,
            Object::List(i, _) => i,
        }
    }

    /// Transforms the object from one information type to another.
    ///
    pub fn transform<U>(&self, f: fn(&T) -> U) -> Object<U> {
        match self {
            Object::Nil(i) => Object::Nil(f(i)),
            Object::Bool(i, v) => Object::Bool(f(i), *v),
            Object::Integer(i, v) => Object::Integer(f(i), *v),
            Object::Float(i, v) => Object::Float(f(i), *v),
            Object::Char(i, v) => Object::Char(f(i), *v),
            Object::String(i, v) => Object::String(f(i), v.clone()),
            Object::Keyword(i, v) => Object::Keyword(f(i), v.clone()),
            Object::Symbol(i, v) => Object::Symbol(f(i), v.clone()),
            Object::List(i, v) => Object::List(f(i), v.iter().map(|x| x.transform(f)).collect()),
        }
    }

    /// Transforms the object tree into a string.
    ///
    pub fn stringify(&self, max_depth: usize, colors: bool) -> String {
        if !colors {
            self.stringify_internal(max_depth, 0)
        } else {
            self.stringify_internal_colored(max_depth, 0)
        }
    }

    fn stringify_internal(&self, max_depth: usize, depth: usize) -> String {
        match self {
            Self::Nil(_) => "nil".to_string(),
            Self::Bool(_, v) => format!("{}", v),
            Self::Integer(_, v) => format!("{}", v),
            Self::Float(_, v) => format!("{}", v),
            Self::Char(_, v) => match v {
                '\n' => "#newline".to_string(),
                '\t' => "#tab".to_string(),
                '\r' => "#carriage-return".to_string(),
                ' ' => "#space".to_string(),
                _ => format!("#{}", v),
            },
            Self::String(_, v) => format!("{:?}", v),
            Self::Keyword(_, v) => format!(":{}", v),
            Self::Symbol(_, v) => v.clone(),
            Self::List(_, v) => {
                if depth >= max_depth {
                    "(...)".to_string()
                } else {
                    let mut r = "(".to_string();

                    for (i, x) in v.iter().enumerate() {
                        r.push_str(&x.stringify_internal(max_depth, depth + 1));

                        if i + 1 < v.len() {
                            r.push(' ');
                        }
                    }

                    r.push(')');
                    r
                }
            }
        }
    }

    fn stringify_internal_colored(&self, max_depth: usize, depth: usize) -> String {
        use colored::*;

        match self {
            Self::Nil(_) => "nil".magenta().bold().to_string(),
            Self::Bool(_, v) => format!("{}", v).magenta().bold().to_string(),
            Self::Integer(_, v) => format!("{}", v).yellow().to_string(),
            Self::Float(_, v) => format!("{}", v).yellow().to_string(),
            Self::Char(_, v) => (match v {
                '\n' => "#newline".to_string(),
                '\t' => "#tab".to_string(),
                '\r' => "#carriage-return".to_string(),
                ' ' => "#space".to_string(),
                _ => format!("#{}", v),
            })
            .yellow()
            .bold()
            .to_string(),
            Self::String(_, v) => format!("{:?}", v).yellow().bold().to_string(),
            Self::Keyword(_, v) => format!(":{}", v).purple().to_string(),
            Self::Symbol(_, v) => v.clone().purple().to_string(),
            Self::List(_, v) => {
                if depth >= max_depth {
                    "(...)".cyan().to_string()
                } else {
                    let mut r = "(".cyan().to_string();

                    for (i, x) in v.iter().enumerate() {
                        r.push_str(&x.stringify_internal_colored(max_depth, depth + 1));

                        if i + 1 < v.len() {
                            r.push(' ');
                        }
                    }

                    r.push_str(&")".cyan().to_string());
                    r
                }
            }
        }
    }
}

//==================================================================================================
// Trait implementations

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Nil => write!(f, "Nil"),
            Kind::Bool => write!(f, "Bool"),
            Kind::Integer => write!(f, "Integer"),
            Kind::Float => write!(f, "Float"),
            Kind::Char => write!(f, "Char"),
            Kind::String => write!(f, "String"),
            Kind::Keyword => write!(f, "Keyword"),
            Kind::Symbol => write!(f, "Symbol"),
            Kind::List => write!(f, "List"),
            Kind::Any => write!(f, "Any"),
        }
    }
}

impl<T> fmt::Display for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil(_) => write!(f, "nil"),
            Object::Bool(_, v) => write!(f, "{}", v),
            Object::Integer(_, v) => write!(f, "{}", v),
            Object::Float(_, v) => write!(f, "{}", v),
            Object::Char(_, v) => match v {
                '\n' => write!(f, "#newline"),
                '\t' => write!(f, "#tab"),
                '\r' => write!(f, "#carriage-return"),
                ' ' => write!(f, "#space"),
                _ => write!(f, "#{}", v),
            },
            Object::String(_, v) => write!(f, "{:?}", v),
            Object::Keyword(_, v) => write!(f, ":{}", v),
            Object::Symbol(_, v) => write!(f, "{}", v),
            Object::List(_, v) => {
                write!(f, "(")?;
                for (i, x) in v.iter().enumerate() {
                    write!(f, "{}", x)?;

                    if i + 1 < v.len() {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}
