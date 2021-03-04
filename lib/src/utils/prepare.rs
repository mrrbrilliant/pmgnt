use std::{fs::create_dir_all, io::Result, path::PathBuf};

pub fn prepare_base(path: PathBuf) -> Result<()> {
    if !path.exists() {
        match create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}
