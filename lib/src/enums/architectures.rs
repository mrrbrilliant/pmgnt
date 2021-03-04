use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Architectures {
    X86,
    X86_64,
    Aarch64,
    Armhf,
}

impl Default for Architectures {
    fn default() -> Self {
        Self::X86_64
    }
}
