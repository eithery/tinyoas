use std::path::Path;
use crate::cli::CliResult;
use crate::oas_path::source::OasSourcePath;
use crate::oas_path::target::OasTargetPath;

pub fn build<T>(source: &Path, dest: &Path) -> CliResult {
    let source_files = source.oas_files()?;
    let result_file = dest.result_file_path()?;

    for file in source_files {
        println!("Process file: {}", file.display())
    }

    // fs::create_dir_all(target.clean())
    //     .or_else(|err| target.invalid_target_directory(err))?;

    println!("Done. Result file: {}", result_file.display());
    Ok(())
}


pub fn inspect<T>(source: &Path) -> CliResult {
    println!("Inspect tinyOAS sources...");
    println!("Source: {:?}", source);
    Ok(())
}
