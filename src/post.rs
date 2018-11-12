use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::posts;


#[derive(Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

#[table_name = "posts"]
#[derive(Deserialize, Insertable)]
pub struct PostE {
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

impl Post {
    pub fn all(conn: &PgConnection) -> Vec<Post> {
        use schema::posts::dsl::*;
        posts.load::<Post>(conn).unwrap()
    }

    pub fn add(p: PostE, conn: &PgConnection) -> bool {
        diesel::insert_into(posts::table).values(&p).execute(conn).is_ok()
    }
}
