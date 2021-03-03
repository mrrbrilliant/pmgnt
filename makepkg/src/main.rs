#![allow(unused_imports)]

pub mod download;
pub mod git;

use colored::{Color, Colorize};
use download::download;
use lazy_static::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use shellfn::shell;
use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, Error as ioError, ErrorKind},
    path::{Path, PathBuf},
    str,
};
use url::{Position, Url};

lazy_static! {
    // Dirs
    static ref BASEDIR: PathBuf = cwd();
    static ref SRCDIR: PathBuf = cwd().join("source");
    static ref PKGDIR: PathBuf = cwd().join("pkg");
    // Files
    static ref PKGFILE: PathBuf = cwd().join("pkgbuild.yml");
    static ref SUFFIX: String = String::from(".app");
    static ref DOTFILES: PathBuf = PKGDIR.join(".files");
    // Structs
    #[derive(Debug)]
    static ref PKGDATA: Result<Package, ioError> = read_pkgbuild();
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
    }
}

#[allow(dead_code)]
fn gen_template() -> Result<(), ioError> {
    let file = File::create("pkgbuild.template.yml");
    let pkg: Package = Package::default();
    match file {
        Ok(f) => match serde_yaml::to_writer(f, &pkg) {
            Ok(_) => Ok(()),
            Err(e) => Err(ioError::new(ErrorKind::Other, e.to_string())),
        },
        Err(e) => Err(e),
    }
}

fn prepare_base(path: PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap()
    }
}

fn read_pkgbuild() -> Result<Package, ioError> {
    match PKGFILE.exists() {
        false => Err(ioError::new(
            ErrorKind::NotFound,
            "No pkgbuild.yml found in current directory",
        )),
        true => {
            let file =
                File::open(PKGFILE.display().to_string()).expect("Unable to read pkgbuild.yml");
            match serde_yaml::from_reader(file) {
                Err(e) => Err(ioError::new(ErrorKind::Other, e.to_string())),
                Ok(pkg) => Ok(pkg),
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
    // println!("{:#?}", PKGDATA.as_ref().unwrap());
    println!("{}", "PULLING RESOURCES".green().bold());
    PKGDATA.as_ref().unwrap().pull_all().await;
    PKGDATA.as_ref().unwrap().build();
    // PKGDATA.as_ref().unwrap()
    // let file_path= SRCDIR.join("exe-thumbnailer-0.10.1-1-any.pkg.tar.xz");
    // download::download(&file_path.to_str().unwrap(), "exe-thumbnailer", "https://repo.koompi.org/packages/exe-thumbnailer-0.10.1-1-any.pkg.tar.xz").await.unwrap()
    use walkdir::WalkDir;

    if DOTFILES.exists() {
        std::fs::remove_file(DOTFILES.to_path_buf()).unwrap()
    }

    let mut files: Vec<String> = Vec::new();

    for entry in WalkDir::new(PKGDIR.to_path_buf()).min_depth(1) {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            let buf = entry
                .path()
                .display()
                .to_string()
                .trim_start_matches(PKGDIR.to_str().unwrap())
                .to_string();
            files.push(buf);
        }
    }
    let buffer: String = files.join("\n");

    let mut dotfile = File::create(DOTFILES.to_path_buf()).unwrap();
    dotfile.write(buffer.as_bytes()).unwrap();
    dotfile.flush().unwrap();

    let archive_name = PKGDATA.as_ref().unwrap().archive_name();
    let pkgf = File::create(&archive_name).unwrap();
    let mut tar = tar::Builder::new(pkgf);
    tar.append_dir_all(".", PKGDIR.to_path_buf()).unwrap();
    compress(&archive_name).unwrap();
    fs::remove_file(&archive_name).unwrap();
}

fn compress(source: &str) -> io::Result<()> {
    let mut file = fs::File::open(source)?;
    let mut encoder = {
        let target = fs::File::create(source.to_string() + &SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

#[shell(cmd = "fakeroot sh -c $MODULE")]
pub fn run(
    module: &str,
    basedir: &str,
    srcdir: &str,
    pkgdir: &str,
    pkgname: &str,
    pkgver: &str,
    pkgrel: u32,
) -> Result<String, Box<dyn Error>> {
    ""
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct Script {
    pub commands: Vec<String>,
}

impl Script {
    pub fn exec(&self) -> Result<(), Box<dyn Error>> {
        // Commands to  execute
        let mut commands = self.commands.clone();
        commands.push(String::from("exit"));
        let cmd = &self.commands.join("\n").to_string();

        // Envronment varialbes
        let basedir = BASEDIR.to_str().unwrap();
        let srcdir = SRCDIR.to_str().unwrap();
        let pkgdir = PKGDIR.to_str().unwrap();
        let pkgname = &PKGDATA.as_ref().unwrap().pkgname;
        let pkgver = &PKGDATA.as_ref().unwrap().pkgver;
        let pkgrel = PKGDATA.as_ref().unwrap().pkgrel;

        // execute commands
        run(cmd, basedir, srcdir, pkgdir, pkgname, pkgver, pkgrel)
            .map(|output| println!("{}", output))
    }
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BuildOption {
    #[serde(rename = "strip")]
    Strip,
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "libtool")]
    Libtool,
    #[serde(rename = "staticlibs")]
    Staticlibs,
    #[serde(rename = "emptydirs")]
    Emptydirs,
    #[serde(rename = "zipman")]
    Zipman,
    #[serde(rename = "purge")]
    Purge,
    #[serde(rename = "debug")]
    Debug,
}

impl Default for BuildOption {
    fn default() -> Self {
        Self::Strip
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
    pub options: Option<Vec<BuildOption>>,
    pub install: Option<String>,
    pub changelog: Option<String>,
    pub source: Option<Vec<String>>,
    pub noextract: Option<Vec<String>>,
    pub md5sums: Option<Vec<String>>,
    pub validpgpkeys: Option<Vec<String>>,
    pub prepare: Option<Script>,
    pub build: Option<Script>,
    pub check: Option<Script>,
    pub package: Script,
}

impl Package {
    pub fn build(&self) {
        if let Some(prepare_script) = &self.prepare {
            println!("{}", "PREPARING BUILD".green().bold());
            prepare_script.exec().unwrap();
        }
        if let Some(build_script) = &self.build {
            println!("{}", "RUNNING BUILD".green().bold());
            build_script.exec().unwrap();
        }
        if let Some(check_script) = &self.check {
            println!("{}", "CHECKING BUILD".green().bold());
            check_script.exec().unwrap();
        }
        println!("{}", "PACKING BUILD".green().bold());
        self.package.exec().unwrap();
    }

    pub async fn pull_one(&self, app_name: &str, path_name: &str, source_address: &str) {
        download::download(path_name, app_name, source_address)
            .await
            .unwrap()
    }

    pub async fn pull_all(&self) {
        if let Some(sources) = &self.source {
            for source in sources {
                let parsed_url = Url::parse(source).expect("Unable to parse URL");
                let file_name = &parsed_url
                    .path_segments()
                    .unwrap()
                    .last()
                    .expect("Cannot get file name for URL");
                let file_path = SRCDIR.join(file_name);

                match parsed_url.scheme() {
                    "git" => {
                        println!("Cloning {}", &parsed_url.to_string());
                        git::git_clone(parsed_url.to_string().as_ref(), file_path.to_str().unwrap())
                    }
                    "http" | "https" => {
                        self.pull_one(
                            file_name,
                            &file_path.to_str().unwrap().to_string(),
                            &parsed_url.to_string(),
                        )
                        .await;
                    }
                    _ => {
                        println!("Unsupported URL")
                    }
                }
            }
        }
    }

    pub fn archive_name(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.pkgname, self.pkgver, self.pkgrel, "x86_64"
        )
    }
    // pub async fn pack() {

    // }
}
