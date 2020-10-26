use diesel::{Queryable, Insertable};
use super::schema::shortcuts;
use serde::Serialize;

#[derive(Queryable,Default)]
pub struct Shortcut {
    pub id: i32,
    pub code: String,
    pub url: String,
    pub visits_count: i32,
}

#[derive(Insertable)]
#[table_name = "shortcuts"]
pub struct NewDatabaseShortcut {
    pub code: String,
    pub url: String,
}

#[derive(FromForm)]
pub struct NewShortcut {
    pub url: String,
}

#[derive(Serialize)]
pub struct NewShortcutTemplateData {
    pub url: String,
}

#[derive(Serialize)]
pub struct ErrorCodeTemplateData {
    pub error_message: String,
    pub error_desc: String,
}