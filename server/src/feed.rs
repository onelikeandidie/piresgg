use chrono::NaiveDate;
use serde::Serialize;
use crate::config::Config;

#[derive(Default, Serialize)]
pub struct Feed {
    pub title: String,
    pub subtitle: Option<String>,
    pub xml: String,
    pub link: String,
    pub id: String,
    pub updated: String,
}

impl From<Config> for Feed {
    fn from(config: Config) -> Self {
        let host = config.host;
        let title = config.feed.title;
        let subtitle = config.feed.subtitle;
        let id = config.feed.id;
        
        let xml = format!("{}/feed.atom", host);
        let link = host;
        
        let updated = chrono::Utc::now().to_rfc3339();

        Self {
            title,
            subtitle,
            xml,
            link,
            id,
            updated,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub alternate: Option<String>,
    pub edit: Option<String>,
    pub id: String,
    pub published: Option<NaiveDate>,
    pub updated: Option<NaiveDate>,
    pub summary: String,
    pub content: String,
    pub author: Author,
}

#[derive(Debug, Default, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}
