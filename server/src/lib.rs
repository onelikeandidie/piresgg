use std::{
    collections::HashMap, ffi::OsStr, path::{Path, PathBuf}
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod feed;
pub mod helpers;
pub mod html;
pub mod states;

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub slug: String,
    pub description: String,
    #[serde(default)]
    pub hidden: bool,
    // TODO: Make this work with web states
    // #[serde(default)]
    // pub show_on_nav: bool,
}

impl Post {
    pub fn all(path: &str) -> HashMap<String, Self> {
        let path = Path::new(path);
        let mut posts = HashMap::new();

        for entry in walkdir::WalkDir::new(path) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let entry = entry.path();
                if entry.extension() == Some(OsStr::new("md")) {
                    let post = Post::from(entry.to_path_buf());
                    posts.insert(
                        post.meta.slug.clone(),
                        post,
                    );
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Loaded Posts");
            for (name, post) in posts.clone() {
                println!("{}: {}", name, post.meta.title);
            }
        }

        posts
    }
}

impl From<PathBuf> for Post {
    fn from(path: PathBuf) -> Self {
        let content = std::fs::read_to_string(&path).unwrap();
        let meta_file = path.with_extension("toml");
        let meta =
            toml::from_str::<PostMeta>(&std::fs::read_to_string(&meta_file).unwrap()).unwrap();

        Self { meta, content }
    }
}