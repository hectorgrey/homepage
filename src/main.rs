#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

use rocket_db_pools::{sqlx, Connection, Database};
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
async fn display_article(db: Connection<Blog>, article_index: i64) -> Template {
	let article = article::Article::read(db, article_index).await;
	if let Some(article) = article {
		Template::render(
			"article",
			context!(
				title: article.title,
				content: article.content,
				category: article.category.to_string()),
		)
	} else {
		Template::render("article_not_found", context!())
	}
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
