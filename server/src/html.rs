use pulldown_cmark::{Event, Parser};

pub fn add_classes(parser: Parser) -> Vec<Event>
{
    parser.map(|event| {
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
                    tag => Event::Start(tag)
                }
            }
            _ => event
        }
    }).collect()
}