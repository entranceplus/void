use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::posts::dsl::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

impl Post {
    pub fn all(conn: &PgConnection) -> Vec<Post> {
        posts.limit(2)
            .load::<Post>(conn)
            .expect("Error")
    }
}