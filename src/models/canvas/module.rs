// Auto-generated from native-modules.js
// Source: node_modules\@babel\compat-data\native-modules.js

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// native-modules model - ported from Canvas
/// Reference: node_modules\@babel\compat-data\native-modules.js
pub struct Module {
    // Fields
    pub exports: Option<String>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            exports: None,
        }
    }

}
