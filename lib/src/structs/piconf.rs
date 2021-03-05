use crate::structs::BinRepo;
use serde::{Deserialize, Serialize};

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
}
