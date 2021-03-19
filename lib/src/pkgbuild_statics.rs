use crate::structs::Package;
use lazy_static::*;
use std::{env, io::Error, path::PathBuf};

#[macro_export]
lazy_static! {
    // Dirs
    pub static ref PB_DIR_CWD: PathBuf = cwd();
    pub static ref PB_DIR_SRC: PathBuf = cwd().join("source");
    pub static ref PB_DIR_PKG: PathBuf = cwd().join("pkg");
    // Files
    pub static ref PB_FILE_PKG: PathBuf = cwd().join("pkgbuild.yml");
    pub static ref SUFFIX_APP: String = String::from(".app");
    pub static ref PM_FILE_MANI: PathBuf = PB_DIR_PKG.join("manifest.yml");
    // Structs
    #[derive(Debug)]
    pub static ref DATA_PACKAGE: Result<Package, Error> = Package::from(PB_FILE_PKG.to_path_buf());
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
    }
}
