use std::io;
use std::fmt;
use std::path::Path;
use owo_colors::OwoColorize;
use crate::cli;

pub struct Error {
    message: String,
    inner_error: Option<io::Error>
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_error(f, &self.message, &self.inner_error)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            message: err.to_string(),
            inner_error: None
        }
    }
}


pub(crate) fn _invalid_path_format<T>(path: &Path, kind: &str) -> cli::Result<T> {
    fail(format!("Invalid documents {} path format '{}'", kind, path.display().yellow()), None)
}


pub(crate) fn cannot_access_path<T>(path: &Path, kind: &str, err: io::Error) -> cli::Result<T> {
    fail(format!("Cannot access to OAS documents {} path '{}'", kind, path.display().yellow()), Some(err))
}


pub(crate) fn path_does_not_exist<T>(path: &Path) -> cli::Result<T> {
    fail(format!("Documents source path '{}' does not exist", path.display().yellow()), None)
}


pub(crate) fn path_must_be_directory<T>(path: &Path) -> cli::Result<T> {
    fail(format!("Documents source path '{}' must point to a directory", path.display().yellow()), None)
}


pub(crate) fn no_source_files_found<T>(path: &Path) -> cli::Result<T> {
    fail(format!("No any tinyOAS source files found in '{}'", path.display().yellow()), None)
}


pub(crate) fn not_supported_file_type<T>(path: &Path) -> cli::Result<T> {
    fail(format!("The path '{}' points to not supported file type", path.display().yellow()), None)
}


fn format_error(f: &mut fmt::Formatter<'_>, message: &str, inner_error: &Option<io::Error>) -> fmt::Result {
    writeln!(f, "{} {}", "error:".red(), message)?;
    if let Some(err) = inner_error {
        writeln!(f, "\nCaused by:\n    {}", err)?;
    }
    Ok(())
}


fn fail<T>(message: String, inner_error: Option<io::Error>) -> cli::Result<T> {
    Err(Error { message, inner_error })
}
