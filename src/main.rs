#[macro_use] extern crate rocket;

use rocket::fs::{
    relative,
    FileServer,
};

use rocket_dyn_templates::{
    Template,
    context,
};

#[get("/")]
fn index() -> Template {
    Template::render("index", context!())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
