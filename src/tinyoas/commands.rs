use std::path::PathBuf;
use std::fs;
use owo_colors::OwoColorize;
use crate::errors::*;


pub fn build(_source: &PathBuf, target: &PathBuf) -> CliResult {
    fs::create_dir_all(target)
        .or_error(format!("Unable to create a target directory '{}'", target.display().yellow()))?;

    println!("Done");
    Ok(())
}


pub fn inspect(source: &PathBuf) -> CliResult {
    println!("Inspect tinyOAS sources...");
    println!("Source: {:?}", source);
    Ok(())
}
