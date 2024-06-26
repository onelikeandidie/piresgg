use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::header::{ContentDisposition, DispositionType};
use pulldown_cmark::Parser;
use tera::Context;
use crate::{html, Post};
use crate::config::Config;
use crate::feed::{Author, Entry};
use crate::html::HTMLOptions;
use crate::states::{CacheState, PostsState, TemplateState};

#[get("/")]
pub async fn home(state: web::Data<PostsState>, template: web::Data<TemplateState>) -> impl Responder {
    let posts = {
        let posts = state.posts.lock().unwrap();
        posts.clone()
    };
    let mut context = Context::new();
    let mut posts: Vec<(String, Post)> = posts
        .into_iter()
        .filter(|(_, post)| !post.meta.hidden)
        .collect();
    posts.sort_by(|(_, a), (_, b)| b.meta.date.cmp(&a.meta.date));
    context.insert("posts", &posts);

    let body = template.render("index", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/post/{post}")]
pub async fn render_post(
    req: HttpRequest,
    posts: web::Data<PostsState>,
    cache: web::Data<CacheState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let (cache_hit, (html_output, meta)) = {
        let post_slug = req.match_info().query("post");

        // Check if the post is in the cache
        let cached_html = {
            let cache = cache.post_cache.lock().unwrap();
            cache.get(post_slug).cloned()
        };

        if let Some(cache) = cached_html {
            // Add cache to headers for debugging
            #[cfg(debug_assertions)]
            {
                println!("Cache hit for {}", post_slug);
            }

            (true, cache)
        } else {
            let post = {
                let posts = posts.posts.lock();
                posts.unwrap().get(post_slug).cloned()
            };
            if post.is_none() {
                return HttpResponse::NotFound().reason("Post not found").finish();
            }
            let post = post.unwrap().clone();

            #[cfg(debug_assertions)]
            {
                println!("Rendering {}:{:?}", post_slug, post.meta.clone());
            }

            // Set up options and parser. Strikethrough are not part of the CommonMark standard,
            // and we therefore must enable it explicitly.
            let options = html::get_parser_options();
            let parser = Parser::new_ext(&post.content, options);

            #[cfg(debug_assertions)]
            {
                let options = html::get_parser_options();
                let parser = Parser::new_ext(&post.content, options);
                for event in parser {
                    println!("{:?}", event);
                }
            }

            // Write to String buffer.
            let parser = html::add_classes(parser, HTMLOptions {
                lazy_images: true,
                ..Default::default()
            });
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

            // Add to cache
            {
                let mut cache = cache.post_cache.lock().unwrap();
                cache.insert(
                    post_slug.to_string(),
                    (html_output.clone(), post.meta.clone()),
                );
            }

            (false, (html_output, post.meta.clone()))
        }
    };
    let mut context = Context::new();
    context.insert("meta", &meta);
    context.insert("content", &html_output);
    let mut response = HttpResponse::Ok();
    if cache_hit {
        response.insert_header(("X-Cached-Post", "HIT"));
    };
    response
        .content_type("text/html")
        .body(template.render("post", &context).unwrap())
}

#[get("/tag/{tag}")]
pub async fn serve_tag(
    req: HttpRequest,
    posts: web::Data<PostsState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let tag = req.match_info().query("tag");
    let posts = {
        let posts = posts.posts.lock().unwrap();
        posts.clone()
    };
    let mut posts: Vec<(String, Post)> = posts
        .into_iter()
        .filter(|(_, post)| !post.meta.hidden && post.meta.tags.contains(&tag.to_string()))
        .collect();
    posts.sort_by(|(_, a), (_, b)| b.meta.date.cmp(&a.meta.date));
    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("tag", &tag);
    let body = template.render("tag", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/tags")]
pub async fn serve_tags(
    posts: web::Data<PostsState>,
    cache: web::Data<CacheState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let tags = {
        let tags = cache.tags_cache.lock().unwrap();
        if tags.is_empty() {
            let posts = posts.posts.lock().unwrap();
            // Only show tags for non-hidden posts
            let tags = posts
                .values()
                .filter(|post| !post.meta.hidden)
                .flat_map(|post| post.meta.tags.clone())
                .collect::<Vec<String>>();
            // Count the number of times each tag appears
            let tags: Vec<(String, usize)> = tags
                .iter()
                // This is kindof complicated, the fold function is like an
                // accumulator in a for loop. It takes an initial value and a
                // closure that takes the accumulator and the current value and
                // returns the new accumulator. In this case, we're using it to
                // count the number of times each tag appears.
                .fold(std::collections::HashMap::new(), |mut acc, tag| {
                    *acc.entry(tag.clone()).or_insert(0) += 1;
                    acc
                })
                .into_iter()
                .collect();
            tags
        } else {
            tags.clone()
        }
    };
    let mut context = Context::new();
    context.insert("tags", &tags);
    let body = template.render("tags", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/public/{filename:.*}")]
pub async fn serve_static(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = std::path::Path::new("public").join(path);
    let file = actix_files::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[get("/feed.atom")]
pub async fn feed(
    config_state: web::Data<Config>,
    posts: web::Data<PostsState>,
    cache: web::Data<CacheState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let config = config_state.get_ref().clone();
    let feed = crate::feed::Feed::from(config.clone());

    let posts = {
        let posts = posts.posts.lock().unwrap();
        posts.clone()
    };
    // Check if the post is in the cache
    let cached_html = {
        let cache = cache.feed_post_cache.lock().unwrap();
        cache.clone()
    };

    let posts: Vec<Entry> = posts
        .into_iter()
        .filter(|(_, post)| !post.meta.hidden)
        .map(|(slug, post)| {
            let cached_html = cached_html.get(&slug).cloned();
            let html_output = if let Some(html_output) = cached_html {
                html_output
            } else {
                // Set up options and parser. Strikethroughs are not part of the CommonMark standard
                // and we therefore must enable it explicitly.
                let options = html::get_parser_options();
                let parser = Parser::new_ext(&post.content, options);

                // Write to String buffer.
                let parser = html::add_classes(parser, HTMLOptions {
                    base_url: config_state.host.clone(),
                    ..Default::default()
                });
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

                // Add to cache
                {
                    let mut cache = cache.feed_post_cache.lock().unwrap();
                    cache.insert(slug.to_string(), html_output.clone());
                }

                html_output
            };
            Entry {
                title: post.meta.title,
                link: format!("{}/post/{}", config_state.host, slug),
                alternate: None,
                edit: None,
                id: format!("{}/post/{}", config_state.host, slug),
                published: Some(post.meta.date),
                updated: Some(post.meta.date),
                summary: post.meta.description,
                content: html_output,
                author: Author {
                    name: config.feed.author.name.clone(),
                    email: config.feed.author.email.clone(),
                },
            }
        })
        .collect();
    println!("{:?}", posts);

    let mut context = Context::new();
    context.insert("feed", &feed);
    context.insert("entries", &posts);
    HttpResponse::Ok()
        .content_type("application/atom+xml")
        .body(template.render("atom/feed", &context).unwrap())
}