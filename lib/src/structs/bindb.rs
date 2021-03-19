use crate::structs::{Manifest, Package};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;

use super::BinRepo;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PackageRepo {
    pub name: String,
    pub packages: Vec<Manifest>,
    pub version: u32,
}

impl PackageRepo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();

        let data: PackageRepo = from_reader(file).unwrap();
        data
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SourceRepo {
    pub name: String,
    pub packages: Vec<Package>,
    pub version: String,
}

impl SourceRepo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();

        let data: SourceRepo = from_reader(file).unwrap();
        data
    }
}
