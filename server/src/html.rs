use pulldown_cmark::{Event, Options, Parser, Tag};

#[derive(Default)]
pub struct HTMLOptions {
    pub base_url: String,
    pub lazy_images: bool,
}

pub fn add_classes(parser: Parser, options: HTMLOptions) -> Vec<Event> {
    parser
        .map(|event| {
            match event {
                Event::Start(tag) => {
                    match tag {
                        // Example on how to do this
                        // In case I need it later
                        //
                        // This is how you would add a class to a heading
                        // Tag::Heading {
                        //     level,
                        //     classes,
                        //     attrs,
                        //     id
                        // } => {
                        //     let mut classes = classes.to_owned();
                        //     classes.push("heading".into());
                        //     Event::Start(Tag::Heading {
                        //         level,
                        //         classes,
                        //         attrs,
                        //         id
                        //     })
                        // },
                        Tag::Image { id, title, link_type, dest_url} => {
                            let dest_url = if !dest_url.starts_with("http") {
                                format!("{}/{}", options.base_url, dest_url.trim_start_matches('/'))
                            } else {
                                dest_url.to_string()
                            };
                            if !options.lazy_images {
                                Event::Start(Tag::Image { id, title, link_type, dest_url: dest_url.into() })
                            } else {
                                let dest_url = format!("lazy://{}", dest_url);
                                Event::Start(Tag::Image { id, title, link_type, dest_url: dest_url.into() })
                            }
                        }
                        tag => Event::Start(tag),
                    }
                }
                _ => event,
            }
        })
        .collect()
}


pub fn get_parser_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options
}