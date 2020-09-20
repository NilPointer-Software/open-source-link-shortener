use diesel::{Queryable, Insertable};
use super::schema::shortcuts;
use serde::Serialize;

#[derive(Queryable, Insertable)]
#[table_name = "shortcuts"]
pub struct Shortcut {
    pub code: String,
    pub url: String,
    pub visits_count: i32,
}

#[derive(FromForm)]
pub struct NewShortcut {
    pub url: String,
}

#[derive(Serialize)]
pub struct NewShortcutTemplateData {
    pub url: String,
}