use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RepoProtocol {
    Local,
    Remote,
}

impl Default for RepoProtocol {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self::Local
        } else {
            Self::Remote
        }
    }
}
