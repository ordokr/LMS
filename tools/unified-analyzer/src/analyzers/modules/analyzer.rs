rust
use std::path::PathBuf;
use anyhow::Result;

pub trait Analyzer {
    type Result;
    fn analyze(&self, base_dir: &PathBuf) -> Result<Self::Result>;
}