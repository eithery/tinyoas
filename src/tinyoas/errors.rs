use std::io;
use std::fmt;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use crate::cli;


pub trait CliError {
    fn invalid_source_path<T>(&self, err: io::Error) -> cli::Result<T>;

    fn invalid_target_directory<T>(&self, err: io::Error) -> cli::Result<T>;
}


impl CliError for &PathBuf {
    fn invalid_source_path<T>(&self, err: io::Error) -> cli::Result<T> {
        cli::fail(format!("Invalid tynyOAS documents source path '{}'", self.display().yellow()), err)
    }

    fn invalid_target_directory<T>(&self, err: io::Error) -> cli::Result<T> {
        cli::fail(format!("Unable to create a target directory '{}'", self.display().yellow()), err)
    }
}


pub fn format_error(f: &mut fmt::Formatter<'_>, message: &str, source: &io::Error) -> fmt::Result {
    writeln!(f, "{} {}\n\nCaused by:\n    {}", "error:".red(), message, source)
}
