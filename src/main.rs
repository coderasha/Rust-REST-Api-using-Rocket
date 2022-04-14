#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>,
}
