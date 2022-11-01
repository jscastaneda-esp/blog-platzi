use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

impl Post {
    pub fn slugify(title: &String) -> String {
        title.replace(" ", "to").to_lowercase()
    }

    pub fn create_post<'a>(
        conn: &mut PgConnection,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title);

        let new_post = NewPost {
            title: &post.title,
            body: &post.body,
            slug: &slug,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}
