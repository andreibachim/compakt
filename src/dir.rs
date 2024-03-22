use std::{ffi::OsString, fs};

use anyhow::anyhow;
use lol_html::{element, html_content::ContentType, rewrite_str, RewriteStrSettings};

use crate::{source_dir::SourceDir, target_dir::TargetDir};

pub struct Dir<'a> {
    pub source_directory: SourceDir<'a>,
    pub target_directory: TargetDir<'a>,
    pub inherited_template: Option<String>,
}

impl<'a> Dir<'a> {
    pub fn process(&self) -> anyhow::Result<()> {
        let mut dirs: Vec<OsString> = vec![];
        let mut pages: Vec<String> = vec![];
        let mut layout: Option<String> = None;

        fs::read_dir(self.source_directory.get_path())?
            .flat_map(|r| r.ok())
            .for_each(|dir_entry| {
                if dir_entry.path().is_dir() {
                    dirs.push(dir_entry.file_name());
                }

                if dir_entry.path().is_file() {
                    match dir_entry.file_name().to_string_lossy() {
                        s if s.ends_with("layout.html") => {
                            layout = self.merge_layouts(fs::read_to_string(dir_entry.path()).ok());
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
        println!("{:?}", layout);
        self.process_dirs(dirs, layout)?;

        Ok(())
    }

    fn merge_layouts(&self, inner_layout: Option<String>) -> Option<String> {
        let inherited_layout = self.inherited_template.clone();
        inner_layout.map_or_else(
            || inherited_layout.clone(),
            |inner| {
                Some(
                    inherited_layout
                        .clone()
                        .map(|inherited| inject_into_layout(&inner, &inherited).unwrap())
                        .unwrap_or(inner),
                )
            },
        )
    }

    fn process_dirs(&self, dirs: Vec<OsString>, layout: Option<String>) -> anyhow::Result<()> {
        for dir in dirs {
            let source_directory = self.source_directory.get_path().join(&dir);
            let source_directory = SourceDir::new(&source_directory)?;
            let target_directory = self.target_directory.get_path().join(dir);
            let target_directory = TargetDir::new(&target_directory, false)?;

            Dir {
                source_directory,
                target_directory,
                inherited_template: layout.clone(),
            }
            .process()?;
        }
        Ok(())
    }
}

fn inject_into_layout(content: &str, layout: &str) -> anyhow::Result<String> {
    rewrite_str(
        layout,
        RewriteStrSettings {
            element_content_handlers: vec![element!("[data-slot]", |el| {
                el.replace(&content, ContentType::Html);
                Ok(())
            })],
            ..RewriteStrSettings::default()
        },
    )
    .map_err(|err| anyhow!("Could not inject into layount. Underlying error: {}", err))
}
