#[macro_use] extern crate rocket;

use rocket::fs::{
    relative,
    FileServer,
};

use rocket_dyn_templates::{
    Template,
    context,
};

mod article;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!())
}

#[get("/articles/<article_index>")]
fn display_article(article_index: usize) -> Template {
    Template::render("article", context!(index: article_index))
}

#[get("/rust")]
fn rust() -> Template {
    Template::render("rust", context!())
}

#[get("/linux")]
fn linux() -> Template {
    Template::render("linux", context!())
}

#[get("/freebsd")]
fn freebsd() -> Template {
    Template::render("freebsd", context!())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, rust, linux, freebsd, display_article])
        .mount("/", FileServer::from(relative!("static")))
}
