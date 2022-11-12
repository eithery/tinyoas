use std::path::PathBuf;
use crate::errors::CliError;

pub type CliResult = Result<(), CliError>;


pub fn build(source: &PathBuf, target: &PathBuf) -> CliResult {
    println!("Building OAS3 document...");
    println!("Source: {:?}", source);
    println!("Target: {:?}", target);
    Ok(())
}


pub fn inspect(source: &PathBuf) -> CliResult {
    println!("Inspect tinyOAS sources...");
    println!("Source: {:?}", source);
    Ok(())
}
