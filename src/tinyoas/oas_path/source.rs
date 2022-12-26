use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use crate::cli::{self, CliResult};
use crate::errors;

pub(crate) trait OasSourcePath {
    fn oas_files(&self) -> cli::Result<Vec<PathBuf>>;
}

impl OasSourcePath for Path {
    fn oas_files(&self) -> cli::Result<Vec<PathBuf>> {
        validate(self)?;

        let oas_files: Vec<_> = WalkDir::new(self).into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| oas_file_path(&entry, "yml"))
            .collect();

        if oas_files.is_empty() {
            return errors::no_source_files_found(self);
        }

        Ok(oas_files)
    }
}


fn validate(path: &Path) -> CliResult {
    let path_exists = path.try_exists().or_else(|err| errors::cannot_access_path(path, "source", err))?;
    if !path_exists {
        return errors::path_does_not_exist(path);
    }
    if !path.is_dir() {
        return errors::path_must_be_directory(path);
    }

    Ok(())
}


fn oas_file_path(entry: &DirEntry, file_extension: &str) -> Option<PathBuf> {
    if entry.metadata().unwrap().is_file() {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            return if ext == file_extension { Some(path.to_path_buf()) } else { None };
        }
    }
    None
}
