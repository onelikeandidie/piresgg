pub mod helpers;
pub mod states;

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct PostMeta {
    pub title: String,
}
