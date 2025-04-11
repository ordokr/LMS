// Auto-generated from upgradeinsecurerequests.js
// Source: node_modules\caniuse-lite\data\features\upgradeinsecurerequests.js

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// upgradeinsecurerequests model - ported from Canvas
/// Reference: node_modules\caniuse-lite\data\features\upgradeinsecurerequests.js
pub struct Grade {
    // Fields
    pub A: Option<String>,
    pub B: Option<String>,
    pub C: Option<String>,
    pub D: Option<String>,
    pub E: Option<String>,
    pub F: Option<String>,
    pub G: Option<String>,
    pub H: Option<String>,
    pub I: Option<String>,
    pub J: Option<String>,
    pub K: Option<String>,
    pub L: Option<String>,
    pub M: Option<String>,
    pub N: Option<String>,
    pub O: Option<String>,
    pub P: Option<String>,
    pub Q: Option<String>,
    pub R: Option<String>,
    pub S: Option<String>,
    pub exports: Option<String>,
}

impl Grade {
    pub fn new() -> Self {
        Self {
            A: None,
            B: None,
            C: None,
            D: None,
            E: None,
            F: None,
            G: None,
            H: None,
            I: None,
            J: None,
            K: None,
            L: None,
            M: None,
            N: None,
            O: None,
            P: None,
            Q: None,
            R: None,
            S: None,
            exports: None,
        }
    }

}
