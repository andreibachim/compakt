use std::{fs, path::Path};

use anyhow::{anyhow, Result};

pub struct TargetDir<'a> {
    path: &'a Path,
}

impl<'a> TargetDir<'a> {
    pub fn new(path: &'a Path, overwrite: bool) -> Result<Self> {
        let is_dir = path.is_dir();
        if is_dir && !overwrite {
            return Err(anyhow!(
                "A directory or file already exists at the target location."
            ));
        }

        if is_dir && overwrite {
            if let Err(error) = fs::remove_dir_all(path) {
                return Err(anyhow!(
                    "Could not delete existing directory at target location. Underlying error: {}",
                    error
                ));
            }
        };

        fs::create_dir_all(path)?;

        Ok(Self { path })
    }

    pub fn get_path(&self) -> &Path {
        self.path
    }
}
