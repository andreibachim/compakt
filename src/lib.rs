mod dir;
mod source_dir;
pub mod target_dir;

use std::path::Path;

use dir::Dir;

use crate::{source_dir::SourceDir, target_dir::TargetDir};

pub fn compile(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let source_directory = SourceDir::new(Path::new(options.source_directory))?;
    let target_directory = TargetDir::new(
        Path::new(options.target_directory),
        options.overwrite_target_directory,
    )?;

    Dir {
        source_directory,
        target_directory,
        inherited_template: None,
    }
    .process()?;

    Ok(())
}

#[derive(Default)]
pub struct Options<'a> {
    pub source_directory: &'a str,
    pub target_directory: &'a str,
    pub overwrite_target_directory: bool,
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
