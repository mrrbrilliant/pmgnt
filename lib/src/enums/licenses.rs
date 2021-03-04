use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Licenses {
    Apache2,
    BSD2,
    BSD3,
    GPL,
    LGPL,
    MIT,
    MPL2,
    CDDL,
    EPL2,
}

impl Default for Licenses {
    fn default() -> Self {
        Self::GPL
    }
}
