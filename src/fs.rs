use std::fs;
use std::path::PathBuf;

pub fn mkdir_p(path: PathBuf) -> Result<(), std::io::Error> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}
