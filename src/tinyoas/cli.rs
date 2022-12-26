use crate::errors;

pub type Result<T> = std::result::Result<T, Error>;
pub type CliResult = Result<()>;
pub type Error = errors::Error;
