use std::{collections::HashMap, ffi::OsString, path::Path, sync::Mutex};

use crate::Post;

pub struct TemplateState {
    tera: tera::Tera,
}

impl TemplateState {
    pub fn new(templates: &str) -> Self {
        let mut tera = tera::Tera::default();
        let ext = [".html.tera", ".html", ".tera", ".xml"];
        let mut files: Vec<(String, Option<String>)> = Vec::new();
        tera.autoescape_on(ext.to_vec());

        let templates = Path::new(templates);
        for entry in walkdir::WalkDir::new(templates) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let entry = entry.path();
                // Check if the file extension is in the template list
                let is_template = ext.into_iter().find(|ext| {
                    let into: OsString = ext[1..].to_string().into();
                    entry.extension() == Some(&into)
                });
                // Transform into a tuple for Tera
                if is_template.is_some() {
                    let path = entry.to_string_lossy().to_string();
                    let mut file_name = entry.file_name().unwrap().to_string_lossy().to_string();
                    // Remove the extension from the file name
                    for ext in ext {
                        file_name = file_name.replace(ext, "");
                    }
                    // Add the parent
                    if let Some(parent) = entry.parent() {
                        let parent_relative = parent
                            .to_string_lossy()
                            .to_string()
                            .replace(&templates.to_string_lossy().to_string(), "");
                        let parent_relative =
                            parent_relative.trim_start_matches(std::path::MAIN_SEPARATOR_STR);
                        if !parent_relative.is_empty() {
                            file_name = format!("{}/{}", parent_relative, file_name);
                        }
                    }

                    files.push((path, Some(file_name)))
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Loaded Templates");
            for (path, name) in files.clone() {
                println!("{} -> {}", name.unwrap(), path);
            }
        }

        tera.add_template_files(files).unwrap();

        TemplateState { tera }
    }
    pub fn render(&self, template: &str, context: &tera::Context) -> Result<String, tera::Error> {
        let context = context.clone();
        // Add any needed variables
        self.tera.render(template, &context)
    }
}

pub struct PostsState {
    pub posts: Mutex<HashMap<String, Post>>,
}

#[derive(Default)]
pub struct CacheState {
    pub cache: Mutex<HashMap<String, String>>,
}
