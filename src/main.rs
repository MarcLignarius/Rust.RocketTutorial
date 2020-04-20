#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct IndexContext {
    header: String,
}

#[get("/")]
fn index() -> Template {
    let context = IndexContext {
        header: "Hello!".to_string(),
    };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
