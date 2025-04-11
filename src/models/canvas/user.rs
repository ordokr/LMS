// Auto-generated from user_defined_metadata.d.ts
// Source: node_modules\@tensorflow\tfjs\node_modules\@tensorflow\tfjs-layers\dist\user_defined_metadata.d.ts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// user_defined_metadata model - ported from Canvas
/// Reference: node_modules\@tensorflow\tfjs\node_modules\@tensorflow\tfjs-layers\dist\user_defined_metadata.d.ts
pub struct User {
    // Fields
    pub Default: Option<String>,
    pub https: Option<i64>,
    pub modelName: Option<String>,
    pub name: Option<String>,
    pub userDefinedMetadata: Option<String>,
    pub x: Option<bool>,
}

impl User {
    pub fn new() -> Self {
        Self {
            Default: None,
            https: 0,
            modelName: String::new(),
            name: None,
            userDefinedMetadata: None,
            x: false,
        }
    }

}
