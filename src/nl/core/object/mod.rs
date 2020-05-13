mod boolean;
mod character;
mod float;
mod info;
mod integer;
mod keyword;
mod list;
mod nil;
mod string;
mod symbol;

pub use boolean::*;
pub use character::*;
pub use float::*;
pub use info::*;
pub use integer::*;
pub use keyword::*;
pub use list::*;
pub use nil::*;
pub use string::*;
pub use symbol::*;

use std::fmt;

/// A LISP object.
///
/// It is only used during Compile Time Evaluation.
/// It represents anything that can be encountered is a well formatted lisp source file.
///
#[derive(Clone)]
pub enum Object {
    /// A "no-value", null pointer, or empty list.
    Nil(ObjectInfo),
    /// A boolean.
    Bool(ObjectInfo, bool),
    /// An integer.
    Integer(ObjectInfo, i32),
    /// A float.
    Float(ObjectInfo, f32),
    /// A single character.
    Char(ObjectInfo, char),
    /// A string.
    String(ObjectInfo, String),
    /// A keyword.
    Keyword(ObjectInfo, String),
    /// A symbol.
    Symbol(ObjectInfo, String),
    /// A list.
    List(ObjectInfo, Vec<Object>),
}

impl Object {
    /// Gets the type of the object as a string.
    ///
    pub fn type_string(&self) -> &'static str {
        match self {
            Self::Nil(_) => "Nil",
            Self::Bool(_, _) => "Bool",
            Self::Integer(_, _) => "Integer",
            Self::Float(_, _) => "Float",
            Self::Char(_, _) => "Char",
            Self::String(_, _) => "String",
            Self::Keyword(_, _) => "Keyword",
            Self::Symbol(_, _) => "Symbol",
            Self::List(_, _) => "List",
        }
    }

    /// Gets the information associated with the object.
    ///
    pub fn get_info(&self) -> &ObjectInfo {
        match self {
            Self::Nil(i) => i,
            Self::Bool(i, _) => i,
            Self::Integer(i, _) => i,
            Self::Float(i, _) => i,
            Self::Char(i, _) => i,
            Self::String(i, _) => i,
            Self::Keyword(i, _) => i,
            Self::Symbol(i, _) => i,
            Self::List(i, _) => i,
        }
    }

    /// Gets the information associated with the object.
    ///
    pub fn get_info_mut(&mut self) -> &mut ObjectInfo {
        match self {
            Self::Nil(i) => i,
            Self::Bool(i, _) => i,
            Self::Integer(i, _) => i,
            Self::Float(i, _) => i,
            Self::Char(i, _) => i,
            Self::String(i, _) => i,
            Self::Keyword(i, _) => i,
            Self::Symbol(i, _) => i,
            Self::List(i, _) => i,
        }
    }

    /// Dumps the object and its content to the standard output.
    ///
    pub fn dump(&self) {
        self._dump("".to_string())
    }

    /// Dumps the object and its content to the standard output.
    /// Internal implementation.
    ///
    fn _dump(&self, i: String) {
        match self {
            Self::Nil(_) => println!("{}Nil", i),
            Self::Bool(_, v) => println!("{}Bool : {}", i, v),
            Self::Integer(_, v) => println!("{}Integer : {}", i, v),
            Self::Float(_, v) => println!("{}Float : {}", i, v),
            Self::Char(_, v) => println!("{}Char : {:?}", i, v),
            Self::String(_, v) => println!("{}String : {:?}", i, v),
            Self::Keyword(_, v) => println!("{}Keyword : {}", i, v),
            Self::Symbol(_, v) => println!("{}Symbol : {}", i, v),
            Self::List(_, v) => {
                if v.len() == 0 {
                    println!("{}Empty list", &i);
                } else {
                    println!("{}List : ", &i);
                    for x in v {
                        x._dump(format!("{}  ", i))
                    }
                }
            }
        }
    }

    pub fn complete_location(&mut self, file: &String, source: &String) {
        self.get_info_mut().location.as_direct_mut().and_then(|x| {
            x.complete(file, source);
            Some(true)
        });

        self.as_list_mut().and_then(|x| {
            for y in x {
                y.complete_location(file, source)
            }
            Some(true)
        });
    }
}

impl fmt::Display for Object {
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
                    x.fmt(f)?;

                    if i + 1 < v.len() {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}
