// use anyhow::anyhow;
// use colored::{Color, Colorize};
// use git2::Repository;
// use indicatif::{ProgressBar, ProgressStyle};
// use lazy_static::*;
// use reqwest::{header, Client};
// use serde::{Deserialize, Serialize};
// use serde_yaml;
//
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

use std::{
    fs::{metadata, File},
    io::prelude::*,
    path::Path,
};

pub fn read_to_vec_u8(filename: &Path) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
