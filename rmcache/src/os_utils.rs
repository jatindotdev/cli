use std::fs;

use crate::{cli::CLI, log_error};
use color_print::cprintln;

pub fn remove_entry(entry: fs::DirEntry, cli: &CLI) -> Result<(), ()> {
    let path = entry.path();
    let file_type = entry.file_type().map_err(|_| {
        if cli.flags.verbose {
            log_error!("Failed to get file type: {:?}", path);
        }
        ()
    })?;

    if file_type.is_dir() {
        fs::remove_dir_all(&path).map_err(|_| {
            if cli.flags.verbose {
                log_error!("Failed to remove directory: {:?}", path);
            }
            ()
        })?;
    } else if file_type.is_file() {
        fs::remove_file(&path).map_err(|_| {
            if cli.flags.verbose {
                log_error!("Failed to remove file: {:?}", path);
            }
            ()
        })?;
    }

    Ok(())
}
