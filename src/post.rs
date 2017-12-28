use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::posts;

#[table_name = "posts"]
#[derive(Queryable, Insertable)]
pub struct Post {
    pub id: Option<i32>,
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostE {
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

impl Post {
//    pub fn all(conn: &PgConnection) -> Vec<Post> {
//        posts.limit(2)
//            .load::<Post>(*conn)
//            .expect("Error")
//    }

    pub fn add(p: PostE, conn: &PgConnection) -> bool {
        let post = Post { id: None, url: p.url, title: p.title, tags: p.tags, content: p.content };
        diesel::insert_into(posts::table).values(&post).execute(conn).is_ok()
    }
}