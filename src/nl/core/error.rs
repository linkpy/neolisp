pub use super::location::*;

pub struct Error {
    message: String,
    frames: Vec<Frame>,
}

struct Frame {
    name: String,
    location: Location,
}

impl Error {
    /// Creates a new error.
    ///
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            frames: Vec::new(),
        }
    }

    /// Creates a new error an wraps it into a `Result::Err`.
    pub fn err<T>(message: &str) -> Result<T, Self> {
        Err(Self::new(message))
    }

    pub fn newf(message: &str, name: &str, location: Location) -> Self {
        Self::new(message).push(name, location)
    }

    pub fn errf<T>(message: &str, name: &str, location: Location) -> Result<T, Error> {
        Err(Self::newf(message, name, location))
    }

    pub fn rethrow<T>(e: Result<T, Error>, name: &str, location: Location) -> Result<T, Error> {
        match e {
            Err(v) => v.push_err(name, location),
            x => x,
        }
    }

    /// Pushes a stack frame in the error.
    ///
    pub fn push(mut self, name: &str, location: Location) -> Self {
        self.frames.push(Frame {
            name: name.to_string(),
            location,
        });

        self
    }

    /// Pushes a stack frame in the error and wraps it into a `Result::Err`.
    ///
    pub fn push_err<T>(self, name: &str, location: Location) -> Result<T, Self> {
        Err(self.push(name, location))
    }

    /// Converts the error into a string.
    ///
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        res.push_str(&format!("Evaluation error : {}\n\n", self.message));
        res.push_str("Stacktrace :\n");

        self.frames.iter().enumerate().fold(res, |r, (i, x)| {
            if i == 0 {
                r + &format!("  >>>> : '{}' {}\n", x.name, x.location)
            } else {
                r + &format!("  {:0>4} : '{}' {}\n", i, x.name, x.location)
            }
        })
    }

    /// Converts the error into a string.
    ///
    pub fn to_string_light(&self) -> String {
        let mut res = String::new();

        res.push_str(&format!("Evaluation error : {}\n\n", self.message));
        res.push_str("Stacktrace :\n");

        self.frames
            .iter()
            .filter(|x| !x.name.starts_with('#'))
            .enumerate()
            .fold(res, |r, (i, x)| {
                if i == 0 {
                    r + &format!("  >>>> : '{}' {}\n", x.name, x.location)
                } else {
                    r + &format!("  {:0>4} : '{}' {}\n", i, x.name, x.location)
                }
            })
    }
}
