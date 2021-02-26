use lazy_static::*;
use std::{path::{Path, PathBuf}, env};

lazy_static! {
    static ref BASEDIR: PathBuf = cwd();
    static ref SRCDIR: PathBuf = cwd().join("source");
    static ref PKGDIR: PathBuf = cwd().join("pkg");
    static ref PKGFILE: PathBuf = cwd().join("pkgbuild.yml");
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("work")
    } else {
        env::current_dir().unwrap()
    }
}

fn main() {
    println!("{}", BASEDIR.display());
    println!("{}", BASEDIR.display());
    println!("{}", BASEDIR.display());
    println!("{}", BASEDIR.display());
}

fn config() {}
