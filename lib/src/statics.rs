use crate::structs::{Manifest, Package};
use lazy_static::*;
use solvent::DepGraph;
use std::sync::Mutex;
use std::{env, io::Error, path::PathBuf};

#[macro_export]
lazy_static! {
    // Suffixes
    pub static ref SUFFIX_DB: String = String::from(".db");
    pub static ref SUFFIX_APP: String = String::from(".app");

    // Package Manager
    pub static ref PM_DIR_ROOT: PathBuf = root();
    pub static ref PM_DIR_LIB: PathBuf = PM_DIR_ROOT.join("var/lib/pi");
    pub static ref PM_DIR_LOCAL: PathBuf = PM_DIR_LIB.join("local");
    pub static ref PM_DIR_SYNC: PathBuf = PM_DIR_LIB.join("sync");
    pub static ref PM_DIR_BACK: PathBuf = PM_DIR_LIB.join("backup");
    pub static ref PM_DIR_CONF: PathBuf = PM_DIR_ROOT.join("etc/pi.conf.d");

    pub static ref PM_FILE_CONF: PathBuf = PM_DIR_CONF.join("pi.conf");
    pub static ref PM_FILE_MANI: PathBuf = PB_DIR_PKG.join("manifest.yml");

    #[derive(Debug)]
    pub static ref DATA_MANIFEST: Result<Manifest, Error> = Manifest::from_file(PB_FILE_PKG.to_path_buf());

    // Package Builder
    pub static ref PB_DIR_CWD: PathBuf = cwd();
    pub static ref PB_DIR_SRC: PathBuf = cwd().join("source");
    pub static ref PB_DIR_PKG: PathBuf = cwd().join("package");

    pub static ref PB_FILE_PKG: PathBuf = cwd().join("pkgbuild.yml");

    #[derive(Debug)]
    pub static ref DATA_PACKAGE: Result<Package, Error> = Package::from_file(PB_FILE_PKG.to_path_buf());

    // Dependencies
    pub static ref DEPGRAPH: Mutex<DepGraph<String>> = Mutex::new(DepGraph::new());

}

fn root() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs")
    } else {
        PathBuf::from("/")
    }
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
    }
}
