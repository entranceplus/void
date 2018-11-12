#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_diesel;

extern crate serde;

#[macro_use]
extern crate serde_derive;

use self::diesel::prelude::*;


use diesel::pg::PgConnection;

use dotenv::dotenv;
use std::env;

use rocket_contrib::json::{Json};

mod post;
mod schema;
mod db;

#[cfg(test)]
mod tests;

use self::post::*;

#[derive (Serialize, Deserialize)]
struct GenResponse {
    message: &'static str,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, guy!"
}

#[post("/", format = "application/json", data = "<post_json>")]
fn new(post_json: Json<PostE>, conn: db::Conn) -> Json<GenResponse> {
    let post = post_json.into_inner();
    let response = if Post::add(post, &conn) {
        GenResponse { message: "Posts added successfully", }
    } else {
         GenResponse { message: "Not gonna happen", }
    };
    Json(response)
}

#[get("/")]
fn all(conn: db::Conn) -> Json<Vec<Post>> {
    Json(Post::all(&conn))
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok();

    let pool = db::init_pool();

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index])
        .mount("/posts/", routes![new, all])
        .launch();
}
