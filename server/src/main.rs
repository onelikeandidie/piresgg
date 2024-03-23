use std::{path::PathBuf, sync::Mutex};

use actix_files::NamedFile;
use actix_web::{get, http::header::{ContentDisposition, DispositionType}, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use blog_server::states::{CacheState, TemplateState};
use serde::Deserialize;
use tera::Context;

#[get("/")]
async fn hello(template: web::Data<TemplateState>) -> impl Responder {
    let body = template.render("index", &Context::new()).unwrap();
    HttpResponse::Ok().body(body)
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
    let cache = CacheState {
        posts: Mutex::new(Vec::new()),
    };
    let template_state = TemplateState::new(&templates);

    let cache_state = web::Data::new(cache);
    let template_state = web::Data::new(template_state);
    HttpServer::new(move || {
        App::new()
            .app_data(cache_state.clone())
            .app_data(template_state.clone())
            .service(serve_static)
            .service(hello)
    })
    .bind((
        config.address.unwrap_or("127.0.0.1".to_string()),
        config.port.unwrap_or(8000),
    ))?
    .run()
    .await
}
