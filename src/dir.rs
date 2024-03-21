use std::fs;

use crate::{source_dir::SourceDir, target_dir::TargetDir};

pub struct Dir<'a> {
    pub source_directory: SourceDir<'a>,
    pub target_directory: TargetDir<'a>,
    pub inherited_template: Option<String>,
}

impl<'a> Dir<'a> {
    pub fn process(&self) -> anyhow::Result<()> {
        let parsed_dir = self.parse()?;
        Ok(())
    }

    fn parse(&self) -> anyhow::Result<ParsedDir> {
        let mut dirs: Vec<String> = vec![];
        let mut pages: Vec<String> = vec![];
        let mut inner_template: Option<String> = None;

        fs::read_dir(self.source_directory.get_path())?
            .flat_map(|r| r.ok())
            .for_each(|dir_entry| {
                if dir_entry.path().is_dir() {
                    dirs.push(dir_entry.path().to_string_lossy().to_string());
                }

                if dir_entry.path().is_file() {
                    match dir_entry.file_name().to_string_lossy() {
                        s if s.ends_with("layout.html") => {
                            inner_template = fs::read_to_string(dir_entry.path()).ok();
                        }
                        s if s.ends_with("page.html") => {
                            if let Ok(page_content) = fs::read_to_string(dir_entry.path()) {
                                pages.push(page_content);
                            }
                        }
                        _ => println!("found a normal file!"),
                    }
                }
            });

        Ok(ParsedDir {
            inherited_template: self.inherited_template.clone(),
            inner_template,
            pages,
            dirs,
        })
    }
}

struct ParsedDir {
    inherited_template: Option<String>,
    inner_template: Option<String>,
    pages: Vec<String>,
    dirs: Vec<String>,
}
