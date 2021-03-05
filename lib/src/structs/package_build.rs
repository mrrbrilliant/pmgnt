use crate::enums::{Architectures, BuildOptions, Licenses};
use crate::pkgbuild_statics::*;
use crate::structs::Script;
use crate::utils::archive::create_archive;
use crate::utils::{
    download::{download_git, download_http},
    read_file::read_to_vec_u8,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use url::Url;
use walkdir::WalkDir;

use super::Manifest;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub epoch: Option<u32>,
    pub pkgdesc: String,
    pub arch: Vec<Architectures>,
    pub url: String,
    pub license: Vec<Licenses>,
    pub groups: Option<Vec<String>>,
    pub depends: Option<Vec<String>>,
    pub makedepends: Option<Vec<String>>,
    pub checkdepends: Option<Vec<String>>,
    pub optdepends: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub replaces: Option<Vec<String>>,
    pub backup: Option<Vec<String>>,
    pub options: Option<Vec<BuildOptions>>,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(path: PathBuf) -> Result<Self, Error> {
        match path.exists() {
            false => Err(Error::new(
                ErrorKind::NotFound,
                "No pkgbuild.yml found in current directory",
            )),
            true => {
                let file =
                    File::open(PKGFILE.display().to_string()).expect("Unable to read pkgbuild.yml");
                match serde_yaml::from_reader(file) {
                    Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
                    Ok(pkg) => Ok(pkg),
                }
            }
        }
    }

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
        download_http(path_name, app_name, source_address)
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
                        download_git(parsed_url.to_string().as_ref(), file_path.to_str().unwrap())
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
        if MANIFEST.exists() {
            std::fs::remove_file(MANIFEST.to_path_buf()).unwrap()
        }

        let mut files: Vec<String> = Vec::new();

        for entry in WalkDir::new(PKGDIR.to_path_buf()).min_depth(1) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let shahash = read_to_vec_u8(&entry.path());

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

    pub fn to_manifest(&self) -> Manifest {
        Manifest {
            pkgname: self.pkgname.clone(),
            pkgver: self.pkgver.clone(),
            pkgrel: self.pkgrel.clone(),
            epoch: self.epoch.clone(),
            pkgdesc: self.pkgdesc.clone(),
            arch: self.arch.clone(),
            url: self.url.clone(),
            license: self.license.clone(),
            groups: self.groups.clone(),
            depends: self.depends.clone(),
            makedepends: self.makedepends.clone(),
            checkdepends: self.checkdepends.clone(),
            optdepends: self.optdepends.clone(),
            provides: self.provides.clone(),
            conflicts: self.conflicts.clone(),
            replaces: self.replaces.clone(),
            backup: self.backup.clone(),
            options: self.options.clone(),
            install: self.install.clone(),
            changelog: self.changelog.clone(),
            source: self.source.clone(),
            noextract: self.noextract.clone(),
            md5sums: self.md5sums.clone(),
            validpgpkeys: self.validpgpkeys.clone(),
            files: self.gen_file_list(),
        }
    }

    pub fn create_package(&self) {
        create_archive(&self, PKGDIR.to_path_buf())
    }
}
