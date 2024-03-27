use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: Option<u16>,
    pub address: Option<String>,
    pub templates: Option<String>,
    pub content: Option<String>,
    pub feed: FeedConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FeedConfig {
    pub title: String,
    pub subtitle: Option<String>,
    pub id: String,
    pub author: AuthorConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuthorConfig {
    pub name: String,
    pub email: String,
}