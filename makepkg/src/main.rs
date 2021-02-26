#![allow(unused_imports)]

pub mod download;

use download::download;
use lazy_static::*;
use std::{path::{Path, PathBuf}, env, fs::{ self, File}, io::{ self, prelude::*, Error, ErrorKind}};
use serde::{Deserialize, Serialize};
use serde_yaml;
use url::{Url, Position};

lazy_static! {
    // Dirs
    static ref BASEDIR: PathBuf = cwd();
    static ref SRCDIR: PathBuf = cwd().join("source");
    static ref PKGDIR: PathBuf = cwd().join("pkg");
    // Files
    static ref PKGFILE: PathBuf = cwd().join("pkgbuild.yml");
    // Structs
    #[derive(Debug)]
    static ref PKGDATA: Result<Package, Error> = read_pkgbuild();
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("work")
    } else {
        env::current_dir().unwrap()
    }
}

#[allow(dead_code)]
fn gen_template() -> Result<(), io::Error> {
    let file = File::create("pkgbuild.template.yml");
    let pkg: Package = Package::default();
    match file {
        Ok(f) => {
            match serde_yaml::to_writer(f, &pkg) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        },
        Err(e) => Err(e)
    }
}

fn prepare_base(path: PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap()
    }
}

fn read_pkgbuild() -> Result<Package, Error> {
    match PKGFILE.exists() {
        false => Err(Error::new(ErrorKind::NotFound, "No pkgbuild.yml found in current directory")),
        true => {
            let file = File::open(PKGFILE.display().to_string()).expect("Unable to read pkgbuild.yml");
            match serde_yaml::from_reader(file) {
                Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
                Ok(pkg) => Ok(pkg)
            }
        }
    }
}
#[tokio::main]
async fn main() {

    // println!("{}", BASEDIR.display());
    // println!("{}", SRCDIR.display());
    // println!("{}", PKGDIR.display());
    // println!("{}", PKGFILE.display());

    // gen_template().expect("Failed to generate template file!");
    // #[cfg(debug_assertions)]
    // prepare_base(BASEDIR.to_path_buf());

    prepare_base(SRCDIR.to_path_buf());
    prepare_base(PKGDIR.to_path_buf());
    println!("{:#?}", PKGDATA.as_ref().unwrap());

    PKGDATA.as_ref().unwrap().pull_all().await
    // let file_path= SRCDIR.join("exe-thumbnailer-0.10.1-1-any.pkg.tar.xz");
    // download::download(&file_path.to_str().unwrap(), "exe-thumbnailer", "https://repo.koompi.org/packages/exe-thumbnailer-0.10.1-1-any.pkg.tar.xz").await.unwrap()
    
    
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct Script {
    pub commands: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Architecture {
    X86,
    X86_64,
    Aarch64,
    Armhf,
}

impl Default for Architecture {
    fn default() -> Self {
        Self::X86_64
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum License {
    Apache2,
    BSD2,
    BSD3,
    GPL,
    LGPL,
    MIT,
    MPL2,
    CDDL,
    EPL2,
}

impl Default for License {
    fn default() -> Self {
        Self::GPL
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct Package {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub epoch: Option<u32>,
    pub pkgdesc: String,
    pub arch: Vec<Architecture>,
    pub url: String,
    pub license: Vec<License>,
    pub groups: Option<Vec<String>>,
    pub depends: Option<Vec<String>>,
    pub makedepends: Option<Vec<String>>,
    pub checkdepends: Option<Vec<String>>,
    pub optdepends: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub replaces: Option<Vec<String>>,
    pub backup: Option<Vec<String>>,
    pub options: Option<Vec<String>>,
    pub install: Option<String>,
    pub changelog: Option<String>,
    pub source: Option<Vec<String>>,
    pub noextract:  Option<Vec<String>>,
    pub md5sums: Option<Vec<String>>,
    pub validpgpkeys:  Option<Vec<String>>,
    pub prepare: Option<Script>,
    pub build: Option<Script>,
    pub check: Option<Script>,
    pub package: Script,
}

impl Package {
    pub async fn pull_one(&self, app_name: &str, path_name: &str, source_address: &str) {

        download::download(path_name, app_name, source_address).await.unwrap()
    }

    pub async fn pull_all(&self) {
        if let Some(sources) = &self.source {
            for source in sources {
                let parsed_url = Url::parse(source).expect("Unable to parse URL");
                let file_name = &parsed_url.path_segments().unwrap().last().expect("Cannot get file name for URL");
                let file_path = SRCDIR.join(file_name);
                self.pull_one(file_name, &file_path.to_str().unwrap().to_string(),  &parsed_url.to_string()).await

            }
        }
    }
}