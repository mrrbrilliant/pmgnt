use crate::structs::Package;
use lazy_static::*;
use std::{env, io::Error, path::PathBuf};

#[macro_export]
lazy_static! {
    // Dirs
    pub static ref BASEDIR: PathBuf = cwd();
    pub static ref SRCDIR: PathBuf = cwd().join("source");
    pub static ref PKGDIR: PathBuf = cwd().join("pkg");
    // Files
    pub static ref PKGFILE: PathBuf = cwd().join("pkgbuild.yml");
    pub static ref SUFFIX: String = String::from(".app");
    pub static ref DOTFILES: PathBuf = PKGDIR.join(".files");
    // Structs
    #[derive(Debug)]
    pub static ref PKGDATA: Result<Package, Error> = Package::from(PKGFILE.to_path_buf());
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
    }
}
