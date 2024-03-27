use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionType},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use blog_server::config::Config;
use blog_server::feed::{Author, Entry};
use blog_server::{
    html,
    states::{CacheState, PostsState, TemplateState},
    Post,
};
use pulldown_cmark::Parser;
use tera::Context;

#[get("/")]
async fn home(state: web::Data<PostsState>, template: web::Data<TemplateState>) -> impl Responder {
    let posts = {
        let posts = state.posts.lock().unwrap();
        posts.clone()
    };
    let mut context = Context::new();
    let posts: Vec<(String, Post)> = posts
        .into_iter()
        .filter(|(_, post)| !post.meta.hidden)
        .collect();
    context.insert("posts", &posts);

    let body = template.render("index", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/post/{post}")]
async fn render_post(
    req: HttpRequest,
    posts: web::Data<PostsState>,
    cache: web::Data<CacheState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let (cache_hit, (html_output, meta)) = {
        let post_slug = req.match_info().query("post");

        // Check if the post is in the cache
        let cached_html = {
            let cache = cache.cache.lock().unwrap();
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
            let parser = html::add_classes(parser);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

            // Add to cache
            {
                let mut cache = cache.cache.lock().unwrap();
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

#[get("/public/{filename:.*}")]
async fn serve_static(req: HttpRequest) -> actix_web::Result<NamedFile> {
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
async fn feed(
    config_state: web::Data<Config>,
    posts: web::Data<PostsState>,
    cache: web::Data<CacheState>,
    template: web::Data<TemplateState>,
) -> impl Responder {
    let config = config_state.get_ref().clone();
    let feed = blog_server::feed::Feed::from(config.clone());

    let posts = {
        let posts = posts.posts.lock().unwrap();
        posts.clone()
    };
    // Check if the post is in the cache
    let cached_html = {
        let cache = cache.cache.lock().unwrap();
        cache.clone()
    };

    let posts: Vec<Entry> = posts
        .into_iter()
        .filter(|(_, post)| !post.meta.hidden)
        .map(|(slug, post)| {
            let cached_html = cached_html.get(&slug).cloned();
            let html_output = if let Some((html_output, _)) = cached_html {
                html_output
            } else {
                // Set up options and parser. Strikethroughs are not part of the CommonMark standard
                // and we therefore must enable it explicitly.
                let options = html::get_parser_options();
                let parser = Parser::new_ext(&post.content, options);

                // Write to String buffer.
                let parser = html::add_classes(parser);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

                // Add to cache
                {
                    let mut cache = cache.cache.lock().unwrap();
                    cache.insert(slug.to_string(), (html_output.clone(), post.meta.clone()));
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_file = std::env::var("CONFIG").unwrap_or("config.toml".to_string());
    let config_file = std::path::Path::new(&config_file);
    let config: Config = if config_file.exists() {
        let config_file = std::fs::read_to_string(config_file).unwrap();
        toml::from_str(&config_file).unwrap()
    } else {
        Config::default()
    };
    let config_state = web::Data::new(config.clone());

    let templates = config.clone().templates.unwrap_or("frontend/templates".to_string());
    let template_state = TemplateState::new(&templates, config.clone());
    let content = config.content.unwrap_or("content".to_string());
    let all = Post::all(&content);
    let posts = PostsState {
        posts: Mutex::new(all),
    };

    let posts_state = web::Data::new(posts);
    let template_state = web::Data::new(template_state);
    let cache_state = web::Data::new(CacheState::default());
    HttpServer::new(move || {
        App::new()
            .app_data(config_state.clone())
            .app_data(posts_state.clone())
            .app_data(template_state.clone())
            .app_data(cache_state.clone())
            .wrap(actix_web::middleware::Compress::default())
            .service(serve_static)
            .service(render_post)
            .service(feed)
            .service(home)
    })
    .bind((
        config.address.unwrap_or("127.0.0.1".to_string()),
        config.port.unwrap_or(8000),
    ))?
    .run()
    .await
}
