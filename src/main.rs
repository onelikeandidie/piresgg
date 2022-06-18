use rocket::Request;
use rocket::{fs::FileServer, Config};
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

#[get("/", rank = 2)]
fn index() -> Template {
    Template::render("index", context! {
        foo: "bar",
        no_title: true
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {})
}

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    Template::render("404", context! {})
}

#[get("/.well-known/pki-validation/B11BC8138D2D2C04E0D306DB87D53751.txt")]
fn pki_validation() -> &'static str {
    "Ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(Config::figment())
        .mount("/home", routes![index])
        .mount("/", routes![index, about, pki_validation])
        .register("/", catchers![not_found])
        .mount("/static", FileServer::from("static/"))
        .attach(Template::fairing())
}
