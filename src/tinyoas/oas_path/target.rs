use std::path::{Path, PathBuf};
use chrono::Utc;
use crate::cli;
use crate::errors;

pub(crate) trait OasTargetPath {
    fn result_file_path(&self) -> cli::Result<PathBuf>;
}

impl OasTargetPath for Path {
    fn result_file_path(&self) -> cli::Result<PathBuf> {
        let path_exists = self.try_exists().or_else(|err| errors::cannot_access_path(self, "target", err))?;
        if path_exists {
            let attr = self.metadata()?;
            if attr.is_dir() {
                return Ok(self.join(default_oas_file_name()));
            }
            if attr.is_file() {
                return Ok(self.to_path_buf());
            }
            return errors::not_supported_file_type(self)
        }

        errors::not_supported_file_type(self)
    }
}


fn default_oas_file_name() -> String {
    format!("oas_bundle_{}.yml", Utc::now().format("%y%m%d"))
}

// fn validate_destination_path(&self) -> CliResult {
//     else {
//         if let Some(_ext) = self.extension() {

//         }
//     }

//     Ok(())
// }
