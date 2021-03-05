use crate::enums::{Architectures, BuildOptions, Licenses};
use crate::pkgbuild_statics::MANIFEST;
use crate::structs::Package;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Manifest {
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
    pub files: Vec<String>,
}

impl Manifest {
    pub fn from_pkg(pkg: &Package) -> Self {
        Manifest {
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
    pub fn write(&self) -> Result<(), std::io::Error> {
        let f = MANIFEST.to_path_buf();
        let file = File::create(f);
        match file {
            Ok(f) => match serde_yaml::to_writer(f, &self) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}
