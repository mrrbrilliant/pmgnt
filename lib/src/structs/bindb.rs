use crate::structs::{Manifest, Package};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PackageRepo {
    pub name: String,
    pub packages: Vec<Manifest>,
    pub version: u32,
}

impl PackageRepo {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SourceRepo {
    pub name: String,
    pub packages: Vec<Package>,
    pub version: String,
}

impl SourceRepo {
    fn new() -> Self {
        Self::default()
    }
}
