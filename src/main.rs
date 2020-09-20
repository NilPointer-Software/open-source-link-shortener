#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod models;
pub mod schema;
mod host_guard;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket_contrib::templates::Template;
use crate::database::{MainDbConn};
use rocket::request::Form;
use crate::models::{NewShortcut, Shortcut, NewShortcutTemplateData};
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use diesel::RunQueryDsl;
use crate::host_guard::HostHeader;

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

#[post("/new-shortcut", data = "<new_shortcut>")]
fn new_shortcut(db: MainDbConn,host: HostHeader, new_shortcut: Form<NewShortcut>) -> Template {
    println!("Creating new shortcut from url \"{}\"", new_shortcut.url);
    let short_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect();

    let shortcut_data = Shortcut {
        code: short_code,
        url: new_shortcut.url.clone(),
        visits_count: 0,
    };

    diesel::insert_into(schema::shortcuts::table)
        .values(&shortcut_data)
        .execute(&*db)
        .expect("error");

    let template_data = NewShortcutTemplateData {
        url: format!("https://{}/{}",host.0,&shortcut_data.code)
    };
    Template::render("new_shortcut", &template_data)
}

fn main() {
    println!("Connected to database!");
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index,new_shortcut]).launch();
}
