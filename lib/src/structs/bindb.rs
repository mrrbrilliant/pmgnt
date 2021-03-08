#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PackageRepo {
    pub name: String,
    pub packages: Vec<Manifest>,
    pub version: String,
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
