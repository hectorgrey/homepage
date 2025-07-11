#[macro_use]
extern crate rocket;

use rocket::{
	form::{Contextual, Form},
	fs::{relative, FileServer},
	http::Status,
};

use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::{context, Template};

mod article;
use article::Article;

#[derive(Database)]
#[database("sqlite")]
struct Blog(sqlx::SqlitePool);

#[get("/")]
fn index() -> Template {
	Template::render("index", context!())
}

#[get("/articles/<article_index>")]
async fn display_article(db: Connection<Blog>, article_index: i64) -> Template {
	let article = Article::read(db, article_index).await;
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

#[get("/articles/new")]
fn new_article() -> Template {
	Template::render("new_article", context!())
}

#[post("/article", data = "<form>")]
async fn post_article(db: Connection<Blog>, form: Form<Contextual<'_, Article>>) -> Status {
	if let Some(article) = form.value.clone() {
		let result = Article::add(db, article).await;
		match result {
			Ok(()) => Status::Accepted,
			Err(_) => Status::InternalServerError,
		}
	} else {
		Status::BadRequest
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
		.mount(
			"/",
			routes![
				index,
				rust,
				linux,
				freebsd,
				display_article,
				new_article,
				post_article
			],
		)
		.mount("/", FileServer::from(relative!("static")))
}
