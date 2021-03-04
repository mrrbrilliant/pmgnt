#![allow(unused_imports)]

use anyhow::anyhow;
use colored::{Color, Colorize};
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::*;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_yaml;
use sha2::{Digest, Sha256};
use shellfn::shell;
use std::{
    env,
    error::Error,
    fs::{create_dir_all, metadata, remove_file, File, OpenOptions},
    io::{self, prelude::*, Error as ioError, ErrorKind},
    path::{Path, PathBuf},
    str,
};
// use std::{
//     fs::{self, File},
//     io::{prelude::*, Error},
// };
use tokio::{fs, io::AsyncWriteExt};
use url::{Position, Url};
use walkdir::WalkDir;

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

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    prepare_base(BASEDIR.to_path_buf());
    prepare_base(SRCDIR.to_path_buf());
    prepare_base(PKGDIR.to_path_buf());

    println!("{}", "PULLING RESOURCES".green().bold());
    PKGDATA.as_ref().unwrap().pull_all().await;
    PKGDATA.as_ref().unwrap().build();
    // PKGDATA.as_ref().unwrap().gen_file_list();
    PackageManifest::from_pkg(&PKGDATA.as_ref().unwrap())
        .write(PKGDIR.to_path_buf().join("manifest.yaml"))
        .unwrap();
    PKGDATA.as_ref().unwrap().pack();
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
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
        create_dir_all(path).unwrap()
    }
}

pub fn git_clone(url: &str, clone_to: &str) {
    match Repository::clone(url, clone_to) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub async fn download(file_path: &str, app_name: &str, address: &str) -> Result<(), anyhow::Error> {
    let client = Client::new();

    let total_size = {
        let resp = client.head(address).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                address,
                resp.status(),
            ));
        }
    };

    let mut request = client.get(address);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "=> {app} {bar}",
                app = app_name,
                bar = "{wide_msg}[{bar:60.green/blue}] {percent:>3}% {total_bytes:>10}"
            ))
            .progress_chars("#>-"),
    );

    let file = Path::new(file_path);

    if file.exists() {
        let size = file.metadata()?.len().saturating_sub(1);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = request.send().await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .await?;

    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }
    pb.finish();
    Ok(())
}

fn compress(source: &str) -> io::Result<()> {
    let mut file = File::open(source)?;
    let mut encoder = {
        let target = File::create(source.to_string() + &SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

fn get_file_as_byte_vec(filename: &Path) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
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
pub struct Script {
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
pub enum Architecture {
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
pub enum License {
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
pub struct Package {
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
        download(path_name, app_name, source_address).await.unwrap()
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
                        git_clone(parsed_url.to_string().as_ref(), file_path.to_str().unwrap())
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

    pub fn gen_file_list(&self) -> Vec<String> {
        if DOTFILES.exists() {
            std::fs::remove_file(DOTFILES.to_path_buf()).unwrap()
        }

        let mut files: Vec<String> = Vec::new();

        for entry in WalkDir::new(PKGDIR.to_path_buf()).min_depth(1) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let shahash = get_file_as_byte_vec(&entry.path());

                let mut hasher = Sha256::new();
                hasher.update(&shahash);

                let mut buf = entry
                    .path()
                    .display()
                    .to_string()
                    .trim_start_matches(PKGDIR.to_str().unwrap())
                    .to_string();
                buf.push_str(&format!(" {:x}", hasher.finalize()));
                files.push(buf);
            }
        }

        files
    }

    pub fn pack(&self) {
        let archive_name = PKGDATA.as_ref().unwrap().archive_name();
        let pkgf = File::create(&archive_name).unwrap();
        let mut tar = tar::Builder::new(pkgf);
        tar.append_dir_all(".", PKGDIR.to_path_buf()).unwrap();
        compress(&archive_name).unwrap();
        remove_file(&archive_name).unwrap();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageManifest {
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
    pub files: Vec<String>,
}

impl PackageManifest {
    pub fn from_pkg(pkg: &Package) -> Self {
        PackageManifest {
            pkgname: pkg.pkgname.clone(),
            pkgver: pkg.pkgver.clone(),
            pkgrel: pkg.pkgrel.clone(),
            epoch: pkg.epoch.clone(),
            pkgdesc: pkg.pkgdesc.clone(),
            arch: pkg.arch.clone(),
            url: pkg.url.clone(),
            license: pkg.license.clone(),
            groups: pkg.groups.clone(),
            depends: pkg.depends.clone(),
            makedepends: pkg.makedepends.clone(),
            checkdepends: pkg.checkdepends.clone(),
            optdepends: pkg.optdepends.clone(),
            provides: pkg.provides.clone(),
            conflicts: pkg.conflicts.clone(),
            replaces: pkg.replaces.clone(),
            backup: pkg.backup.clone(),
            options: pkg.options.clone(),
            install: pkg.install.clone(),
            changelog: pkg.changelog.clone(),
            source: pkg.source.clone(),
            noextract: pkg.noextract.clone(),
            md5sums: pkg.md5sums.clone(),
            validpgpkeys: pkg.validpgpkeys.clone(),
            files: pkg.gen_file_list(),
        }
    }
    pub fn write(&self, f: PathBuf) -> Result<(), std::io::Error> {
        let file = File::create(f);
        match file {
            Ok(f) => match serde_yaml::to_writer(f, &self) {
                Ok(_) => Ok(()),
                Err(e) => Err(ioError::new(ErrorKind::Other, e.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}
