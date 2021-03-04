// #![allow(unused_imports)]

// use anyhow::anyhow;
// use colored::{Color, Colorize};
// use git2::Repository;
// use indicatif::{ProgressBar, ProgressStyle};

// use reqwest::{header, Client};
// use serde::{Deserialize, Serialize};
// use serde_yaml;
// use sha2::{Digest, Sha256};
// use shellfn::shell;
// use std::{
//     env,
//     error::Error,
//     fs::{create_dir_all, metadata, remove_file, File, OpenOptions},
//     io::{self, prelude::*, Error as ioError, ErrorKind},
//     path::{Path, PathBuf},
//     str,
// };

// use tokio::{fs, io::AsyncWriteExt};
// use url::{Position, Url};
// use walkdir::WalkDir;

// #[tokio::main]
// async fn main() {
//     #[cfg(debug_assertions)]
//     prepare_base(BASEDIR.to_path_buf());
//     prepare_base(SRCDIR.to_path_buf());
//     prepare_base(PKGDIR.to_path_buf());

//     println!("{}", "PULLING RESOURCES".green().bold());
//     PKGDATA.as_ref().unwrap().pull_all().await;
//     PKGDATA.as_ref().unwrap().build();
//     // PKGDATA.as_ref().unwrap().gen_file_list();
//     PackageManifest::from_pkg(&PKGDATA.as_ref().unwrap())
//         .write(PKGDIR.to_path_buf().join("manifest.yaml"))
//         .unwrap();
//     PKGDATA.as_ref().unwrap().pack();
// }

// fn cwd() -> PathBuf {
//     if cfg!(debug_assertions) {
//         env::current_dir().unwrap().join("rootfs/tmp")
//     } else {
//         env::current_dir().unwrap()
//     }
// }

// #[allow(dead_code)]
// fn gen_template() -> Result<(), ioError> {
//     let file = File::create("pkgbuild.template.yml");
//     let pkg: Package = Package::default();
//     match file {
//         Ok(f) => match serde_yaml::to_writer(f, &pkg) {
//             Ok(_) => Ok(()),
//             Err(e) => Err(ioError::new(ErrorKind::Other, e.to_string())),
//         },
//         Err(e) => Err(e),
//     }
// }
pub mod enums;
pub mod pi_statics;
pub mod pkgbuild_statics;
pub mod structs;
pub mod utils;

// pub use enums::{architectures::Architecture, build_options::BuildOption, licenses::License};
// pub use structs::{package_build::Package, package_manifest::Manifest, scripts::Script};
// pub use utils::*;
