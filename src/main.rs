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
use crate::models::{NewShortcut, Shortcut, NewShortcutTemplateData, NewDatabaseShortcut, ErrorCodeTemplateData};
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use diesel::RunQueryDsl;
use crate::host_guard::HostHeader;
use rocket::response::Redirect;
use diesel::prelude::*;
use rocket::Request;
use regex::Regex;

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

#[get("/<request_code>")]
fn redirect_from_code(db: MainDbConn, request_code: String) -> Result<Option<Redirect>, Box<dyn std::error::Error>> {
    use crate::schema::shortcuts::dsl::*;
    use crate::schema::shortcuts;
    use crate::diesel::ExpressionMethods;

    let found_shortcuts: Vec<Shortcut> = shortcuts
        .filter(shortcuts::code.eq(&request_code))
        .limit(1)
        .load::<Shortcut>(&*db)?;

    match found_shortcuts.len() {
        1 => {
            let selected_shortcut = &found_shortcuts[0];
            Ok(Some(Redirect::moved(selected_shortcut.url.clone())))
        }
        _ => {
            Ok(None)
        }
    }
}


#[post("/new-shortcut", data = "<new_shortcut>")]
fn new_shortcut(db: MainDbConn, host: HostHeader, new_shortcut: Form<NewShortcut>) -> Template {
    println!("Creating new shortcut from url \"{}\"", new_shortcut.url);
    let url_regex = Regex::new(
        r#"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#,
    ).unwrap();
    if !url_regex.is_match(&new_shortcut.url) {
        return Template::render(
            "error_code",
            &ErrorCodeTemplateData {
                error_message: "Invalid URL has been provided".to_string(),
                error_desc: format!("\"{}\" doesn't look like an URL", new_shortcut.url),
            },
        );
    }
    let short_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect();

    let shortcut_data = NewDatabaseShortcut {
        code: short_code,
        url: new_shortcut.url.clone(),
    };

    diesel::insert_into(schema::shortcuts::table)
        .values(&shortcut_data)
        .execute(&*db)
        .expect("error");

    let protocol = match host.0 {
        "localhost:8000" => "http",
        _ => "https",
    };

    let template_data = NewShortcutTemplateData {
        url: format!("{}://{}/{}",protocol, host.0, &shortcut_data.code)
    };
    Template::render("new_shortcut", &template_data)
}

fn main() {
    println!("Connected to database!");
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(Template::fairing())
        .register(catchers![ not_found,internal_error])
        .mount("/", routes![index,new_shortcut,redirect_from_code]).launch();
}

#[catch(404)]
fn not_found() -> Template {
    let error_data = ErrorCodeTemplateData{
        error_message: "HTTP 404: Page not found".to_string(),
        error_desc: "Following link shortcut was not found on the server".to_string()
    };
    Template::render("error_code", &error_data)
}

#[catch(500)]
fn internal_error() -> Template {
    let error_data = ErrorCodeTemplateData{
        error_message: "HTTP 500: Internal server error".to_string(),
        error_desc: "Something went really wrong. Please try again later.".to_string()
    };
    Template::render("error_code", &error_data)
}
