use std::fmt;
use std::io;
use owo_colors::OwoColorize;

pub type CliResult = Result<(), CliError>;


pub struct CliError {
    message: String,
    source: io::Error
}

impl CliError {
    fn new(message: String, source: io::Error) -> CliError {
        CliError {
            message,
            source
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}\n\nCaused by:\n    {}", "error:".red(), self.message, self.source)
    }
}


pub trait CliErrorHandler {
    fn or_error(self, message: String) -> CliResult;
}


impl<T> CliErrorHandler for std::io::Result<T> {
    fn or_error(self, message: String) -> CliResult {
        if let Err(err) = self {
            return fail(message, err);
        }
        Ok(())
    }
}


fn fail(message: String, source: io::Error) -> CliResult {
    Err(CliError::new(message, source))
}
