use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::Deserialize;

pub mod helpers;
pub mod states;

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

impl Post {
    pub fn all(path: &str) -> Vec<Self> {
        let path = Path::new(path);
        let mut posts = Vec::new();

        for entry in walkdir::WalkDir::new(path) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let entry = entry.path();
                if entry.extension().unwrap() == "md" {
                    posts.push(Post::from(entry.to_path_buf()));
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Loaded Posts");
            for post in posts.clone() {
                println!("{}", post.meta.title);
            }
        }

        posts
    }
}

impl From<PathBuf> for Post {
    fn from(path: PathBuf) -> Self {
        let content = std::fs::read_to_string(&path).unwrap();
        let meta_file = path.with_extension("toml");
        let meta = toml::from_str::<PostMeta>(&std::fs::read_to_string(&meta_file).unwrap()).unwrap();

        Self { meta, content }
    }
}