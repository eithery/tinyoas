use crate::errors::CliError;

pub type CliResult = Result<(), CliError>;


pub fn build() -> CliResult {
    println!("TODO: Build OAS3 spec is not implemented...");
    Ok(())
}


pub fn validate() -> CliResult {
    println!("TODO: tinyOAS validation is not implemented...");
    Ok(())
}
