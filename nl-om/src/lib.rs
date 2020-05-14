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
