#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate dotenv;

use self::diesel::prelude::*;


use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod post;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, guy!"
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::post::*;

    let connection: PgConnection = establish_connection();

    let results = Post::all(&connection);

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.url);
    }

    //rocket::ignite().mount("/", routes![index]).launch();
}