use flate2::read::GzDecoder;
use lazy_static::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env::args_os;
use std::fs::File;
use std::io::prelude::*;
use std::io::{copy, stdin, stdout};
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs::create_dir_all};
use tar::Archive;

// const SUFFIX: &'static str = ".app";

lazy_static! {
    // Dirs
    static ref ROOT: PathBuf = rd();
    static ref PI_DIR: PathBuf = ROOT.join("var/lib/pi");
    static ref LOCAL_DIR: PathBuf = PI_DIR.join("local");
    static ref SYNC_DIR: PathBuf = PI_DIR.join("sync");
    static ref BACKUP_DIR: PathBuf = PI_DIR.join("backup");
    static ref CONF_DIR: PathBuf = ROOT.join("etc/pi.conf.d");

    // Files
    static ref DB_SUFFIX: String = String::from(".db");
    static ref CONF_FILE: PathBuf = CONF_DIR.join("pi.conf");
    static ref SUFFIX: String = String::from(".app");

}

fn rd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs")
    } else {
        PathBuf::from("/")
    }
}

fn main() {
    // prepare
    prepare();

    // install
    let args: Vec<String> = env::args().collect();
    let arg_file = &args[1];
    let dest = "rootfs";
    xzstd(arg_file).unwrap();
    untar(arg_file, dest).unwrap();

    // register installation

    let filename = Path::new(arg_file.trim_end_matches(SUFFIX.as_str()));

    // let file = File::open(filename).unwrap();
    // let mut a = Archive::new(file);

    // for file in a.entries().unwrap() {
    //     let f = file.unwrap();
    //     println!("{}", f.path().unwrap().display());
    // }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Conf {
    repos: Vec<Repo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repo {
    name: String,
    address: String,
}

impl Conf {
    pub fn gen() -> Self {
        Self {
            repos: vec![Repo::default()],
        }
    }
}

impl Default for Repo {
    fn default() -> Self {
        Self {
            name: String::from("core"),
            address: if cfg!(debug_assertions) {
                format!("http://localhost:3690/x86_64/core",)
            } else {
                String::from("https://repo.koompi.org/x86_64/core")
            },
        }
    }
}

fn prepare() {
    #[cfg(debug_assertions)]
    if !ROOT.as_path().exists() {
        create_dir_all(ROOT.as_path()).unwrap();
    }

    if !PI_DIR.as_path().exists() {
        create_dir_all(PI_DIR.as_path()).unwrap();
    }

    if !LOCAL_DIR.as_path().exists() {
        create_dir_all(LOCAL_DIR.as_path()).unwrap();
    }

    if !SYNC_DIR.as_path().exists() {
        create_dir_all(SYNC_DIR.as_path()).unwrap();
    }

    if !CONF_DIR.as_path().exists() {
        create_dir_all(CONF_DIR.as_path()).unwrap();
    }

    if !CONF_FILE.as_path().exists() {
        let mut file = File::create(CONF_FILE.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &Conf::gen()).unwrap()
    }
}
