#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::{context, Template};

mod article;

#[derive(Database)]
#[database("sqlite")]
struct Blog(sqlx::SqlitePool);

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
		.attach(Blog::init())
		.mount("/", routes![index, rust, linux, freebsd, display_article])
		.mount("/", FileServer::from(relative!("static")))
}
