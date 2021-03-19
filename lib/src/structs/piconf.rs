use crate::structs::BinRepo;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PiConf {
    repos: Vec<BinRepo>,
}

impl PiConf {
    pub fn gen() -> Self {
        Self {
            repos: vec![BinRepo::default()],
        }
    }

    pub fn from_file(path: &str) -> Self {
        let file_path = PathBuf::from(path);
        let file = File::open(file_path).unwrap();
        let data = from_reader(file).unwrap();
        data
    }
}
