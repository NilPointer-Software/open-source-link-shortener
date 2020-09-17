use dotenv::dotenv;
use std::env;


use diesel::SqliteConnection;

#[database("main")]
pub struct MainDbConn(diesel::SqliteConnection);