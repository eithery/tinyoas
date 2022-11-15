use std::io;
use std::fmt;
use crate::errors;

pub type Result<T> = std::result::Result<T, Error>;
pub type CliResult = Result<()>;


pub struct Error {
    message: String,
    source: io::Error
}

impl Error {
    fn new(message: String, source: io::Error) -> Error {
        Error {
            message,
            source
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        errors::format_error(f, &self.message, &self.source)
    }
}


pub fn fail<T>(message: String, source: io::Error) -> Result<T> {
    Err(Error::new(message, source))
}
