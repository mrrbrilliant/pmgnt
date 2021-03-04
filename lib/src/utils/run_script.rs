// use anyhow::anyhow;
// use colored::{Color, Colorize};
// use git2::Repository;
// use indicatif::{ProgressBar, ProgressStyle};
// use lazy_static::*;
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

use shellfn::shell;
use std::error::Error;
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
