use std::path::Path;

use anyhow::{anyhow, Result};

pub struct SourceDir<'a> {
    path: &'a Path,
}

impl<'a> SourceDir<'a> {
    pub fn new(path: &'a Path) -> Result<Self> {
        if !path.is_dir() {
            return Err(anyhow!("Source directory does not exits."));
        }
        Ok(Self { path })
    }

    pub fn get_path(&self) -> &Path {
        self.path
    }
}
