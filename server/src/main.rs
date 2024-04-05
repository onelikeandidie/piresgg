use std::sync::Mutex;

use actix_web::{
    web, App, HttpServer,
};
use blog_server::config::Config;
use blog_server::{
    states::{CacheState, PostsState, TemplateState},
    Post,
    routes::*
};

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
            .service(serve_tag)
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
