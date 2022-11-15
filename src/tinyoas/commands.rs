use std::fs;
use std::path::PathBuf;
use crate::cli::CliResult;
use crate::errors::*;


pub fn build<T>(source: &PathBuf, target: &PathBuf) -> CliResult {
    let _source_path = &source.canonicalize()
        .or_else(|err| source.invalid_source_path(err))?;

    fs::create_dir_all(target)
        .or_else(|err| target.invalid_target_directory(err))?;

//    let target_path = &target.canonicalize().unwrap();

    println!("Done");
    Ok(())
}


pub fn inspect<T>(source: &PathBuf) -> CliResult {
    println!("Inspect tinyOAS sources...");
    println!("Source: {:?}", source);
    Ok(())
}
