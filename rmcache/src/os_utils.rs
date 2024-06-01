use std::{fs, io::Error};

pub fn remove_entry(entry: &fs::DirEntry) -> Result<(), Error> {
    let path = entry.path();
    let file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(e) => {
            return Err(e);
        }
    };

    if file_type.is_dir() {
        match fs::remove_dir_all(&path) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }
    } else if file_type.is_file() {
        match fs::remove_file(&path) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
