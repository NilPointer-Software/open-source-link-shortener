#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template{
    Template::render("index",0)
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index]).launch();
}
