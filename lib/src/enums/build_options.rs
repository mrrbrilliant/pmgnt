use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BuildOptions {
    #[serde(rename = "strip")]
    Strip,
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "libtool")]
    Libtool,
    #[serde(rename = "staticlibs")]
    Staticlibs,
    #[serde(rename = "emptydirs")]
    Emptydirs,
    #[serde(rename = "zipman")]
    Zipman,
    #[serde(rename = "purge")]
    Purge,
    #[serde(rename = "debug")]
    Debug,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self::Strip
    }
}
