#[macro_use] extern crate rocket;

use rocket_dyn_templates::{
    Template,
    context,
};
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    Template::render::<&str, HashMap<String, String>>("index", HashMap::default())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
