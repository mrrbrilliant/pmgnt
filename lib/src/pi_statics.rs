use lazy_static::*;
use solvent::DepGraph;
use std::sync::Mutex;
use std::{env, path::PathBuf};
#[macro_export]
lazy_static! {
    // Dirs
    pub static ref ROOT: PathBuf = rd();
    pub static ref PI_DIR: PathBuf = ROOT.join("var/lib/pi");
    pub static ref LOCAL_DIR: PathBuf = PI_DIR.join("local");
    pub static ref SYNC_DIR: PathBuf = PI_DIR.join("sync");
    pub static ref BACKUP_DIR: PathBuf = PI_DIR.join("backup");
    pub static ref CONF_DIR: PathBuf = ROOT.join("etc/pi.conf.d");

    // Files
    pub static ref DB_SUFFIX: String = String::from(".db");
    pub static ref CONF_FILE: PathBuf = CONF_DIR.join("pi.conf");
    pub static ref SUFFIX: String = String::from(".app");
    // let mut depgraph: DepGraph<&str> = DepGraph::new();
    pub static ref DEPGRAPH: Mutex<DepGraph<String>> = Mutex::new(DepGraph::new());

}

fn rd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs")
    } else {
        PathBuf::from("/")
    }
}
