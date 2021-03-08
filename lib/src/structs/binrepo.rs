use crate::enums::RepoProtocol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinRepo {
    pub name: String,
    pub protocol: RepoProtocol,
    pub address: String,
}

impl Default for BinRepo {
    fn default() -> Self {
        Self {
            name: String::from("core"),
            protocol: RepoProtocol::default(),
            address: if cfg!(debug_assertions) {
                format!("http://localhost:3690/packages",)
            } else {
                String::from("https://store.koompi.org/packages")
            },
        }
    }
}
