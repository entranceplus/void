use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::fmt;
use std::error;

use schema::posts;

use serde_json::{Value, Error};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

#[table_name = "posts"]
#[derive(Debug, Deserialize, Insertable)]
pub struct PostE {
    pub url: String,
    pub title: String,
    pub tags: String,
    pub content: String,
}

// Error types

#[derive(Debug)]
enum CrawlerError {
    Req(reqwest::Error),
    ParseError
}

impl fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CrawlerError::Req(ref err) => err.fmt(f),
            CrawlerError::ParseError => write!(f, "Failed to parse the subreddit")
        }
    }
}

impl error::Error for CrawlerError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CrawlerError::Req(ref err) => Some(err),
            CrawlerError::ParseError => None
        }
    }
}


impl From<reqwest::Error> for CrawlerError {
    fn from(err: reqwest::Error) -> CrawlerError {
        CrawlerError::Req(err)
    }
}


fn fetch<'a>(val: &'a Value, keys: Vec<&str>) -> Option<Vec<&'a str>> {
    keys.iter()
        .map(|k| val.get(k).and_then(|v| v.as_str()))
        .collect()
}

impl Post {
    pub fn all(conn: &PgConnection) -> Vec<Post> {
        use schema::posts::dsl::*;
        posts.load::<Post>(conn).unwrap()
    }

    pub fn add(p: PostE, conn: &PgConnection) -> bool {
        diesel::insert_into(posts::table).values(&p).execute(conn).is_ok()
    }

    pub fn from_vec(tag: &str, post_data: Vec<&str>) -> Option<PostE> {
        match post_data.as_slice() {
            //todo all this to_string() is probably not good
            [url, title, content] => Some(PostE { url: url.to_string(),
                                                  title: title.to_string(),
                                                  tags: tag.to_string(),
                                                  content: content.to_string()}),
            _ => None,
        }
    }

    pub fn from_json(tag: &str, post: &Value) -> Option<PostE> {
        post.get("data")
            .and_then(|p| fetch(p, vec!["url", "title", "selftext"]))
            .and_then(|data| Post::from_vec(tag, data))
    }
}

fn parse_children(sub: &str, children: &Vec<Value>) -> Vec<PostE> {
    children.iter().map(|c| Post::from_json(sub, c)).flatten().collect()
}


fn get_subreddit_posts(sub: &str) -> Result<Vec<PostE>, CrawlerError>{
    let client: reqwest::Client = reqwest::Client::new();
    
    let response: Value = client.get("https://www.reddit.com/r/rust.json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 6.1;) Gecko/20100101 Firefox/13.0.1")
        .send()?
        .json()?;

    // Work needed
    response.get("data")
        .and_then(|d| d.get("children")) // std::option::Option<&serde_json::Value>
        .and_then(|c| c.as_array())
        .map(|children| parse_children(sub, children))
        .ok_or(CrawlerError::ParseError)
}


#[cfg(test)]
mod test {
    use super::*;
   
    extern crate env_logger;

    use std::env;

    #[test]
    fn test_fetch() {
        let test_json = json!({ "a": "atium", "d": "gold", "e": "bronze"});   
        assert_eq!(fetch(&test_json, vec!["a", "d"]), Some(vec!["atium", "gold"]));
        assert_eq!(fetch(&test_json, vec!["a", "b"]), None)
    }
    
    #[test]
    fn test_parser() {
        let _ = env_logger::try_init();

        let posts = get_subreddit_posts("rust")
            .and_then(|p| {
                info!("posts are {:?}", p);
                Ok(p)
            });

        println!("e {}", posts.unwrap().len());

        println!("world");
        assert_eq!(21, 21)
    }
}
