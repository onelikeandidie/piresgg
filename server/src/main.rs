use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{get, http::header::{ContentDisposition, DispositionType}, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use blog_server::{states::{PostsState, TemplateState, CacheState}, Post, html};
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use tera::Context;

#[get("/")]
async fn hello(state: web::Data<PostsState>, template: web::Data<TemplateState>) -> impl Responder {
    let posts = {
        let posts = state.posts.lock().unwrap();
        posts.clone()
    };
    let mut context = Context::new();
    let posts: Vec<(String, Post)> = posts.into_iter()
        .filter(|(_, post)| {
            !post.meta.hidden
        }).collect();
    context.insert("posts", &posts);

    let body = template.render("index", &context).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/post/{post}")]
async fn render_post(req: HttpRequest, posts: web::Data<PostsState>, cache: web::Data<CacheState>, template: web::Data<TemplateState>) -> impl Responder {
    let (cache_hit, html_output) = {
        let post_slug = req.match_info().query("post");

        // Check if the post is in the cache
        let cached_html = {
            let cache = cache.cache.lock().unwrap();
            cache.get(post_slug).map(|s| s.clone())
        };

        if let Some(html_output) = cached_html {
            // Add cache to headers for debugging
            #[cfg(debug_assertions)]
            {
                println!("Cache hit for {}", post_slug);
            }

            (true, html_output)
        } else {
            let post = {
                let posts = posts.posts.lock();
                posts.unwrap().get(post_slug).map(|p| p.clone())
            };
            if post.is_none() {
                return HttpResponse::NotFound()
                    .reason("Post not found")
                    .finish();
            }
            let post = post.unwrap().clone();

            #[cfg(debug_assertions)]
            {
                println!("Rendering {}:{:?}", post_slug, post.meta.clone());
            }

            // Set up options and parser. Strikethroughs are not part of the CommonMark standard
            // and we therefore must enable it explicitly.
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&post.content, options);

            #[cfg(debug_assertions)]
            {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
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
                cache.insert(post_slug.to_string(), html_output.clone());
            }

            (false, html_output)
        }
    };
    let mut context = Context::new();
    context.insert("content", &html_output);
    let mut response = HttpResponse::Ok();
    if cache_hit {
        response.insert_header(("X-Cached-Post", "HIT"));
    };
    response.body(template.render("post", &context).unwrap())
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

#[derive(Debug, Default, Deserialize)]
struct Config {
    port: Option<u16>,
    address: Option<String>,
    templates: Option<String>,
    content: Option<String>,
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

    let templates = config.templates.unwrap_or("frontend/templates".to_string());
    let template_state = TemplateState::new(&templates);
    let all = Post::all(&config.content.unwrap_or("content".to_string()));
    let posts = PostsState {
        posts: Mutex::new(all),
    };

    let posts_state = web::Data::new(posts);
    let template_state = web::Data::new(template_state);
    let cache_state = web::Data::new(CacheState::new());
    HttpServer::new(move || {
        App::new()
            .app_data(posts_state.clone())
            .app_data(template_state.clone())
            .app_data(cache_state.clone())
            .service(serve_static)
            .service(render_post)
            .service(hello)
    })
        .bind((
            config.address.unwrap_or("127.0.0.1".to_string()),
            config.port.unwrap_or(8000),
        ))?
        .run()
        .await
}
