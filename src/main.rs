#[macro_use]
extern crate diesel;

mod models;
mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DB Url config not found");
    let mut conn =
        PgConnection::establish(&database_url).expect("Error establish connection with database");

    use models::{NewPost, Post};
    use schema::posts::dsl::{body, id, posts, title};

    println!("INSERT INTO");
    let new_post = NewPost {
        title: "Nuevo",
        body: "LoreExcepteur irure reprehenderit veniam veniam eu Lorem elit minim in ea laborum cillum sint.",
        slug: "Ut nulla proident cupidatat aliquip voluptate Lorem officia. Mollit ullamco officia magna et irure pariatur Lorem eiusmod reprehenderit. Deserunt amet quis aliquip qui consectetur veniam ex. Sunt dolore nulla consectetur nulla magna. Officia esse nostrud officia deserunt aute cillum voluptate sint Lorem."
    };
    diesel::insert_into(posts)
        .values(&new_post)
        .get_result::<Post>(&mut conn)
        .expect("Error insertion record");

    println!("\nSELECT * FROM posts");
    let posts_result = posts.load::<Post>(&mut conn).expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nSELECT * FROM posts LIMIT 1");
    let posts_result = posts
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nSELECT title, body FROM posts");
    let posts_result = posts
        .select((title, body))
        .load::<(String, String)>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nSELECT * FROM posts ORDER BY id DESC");
    let posts_result = posts
        .order(id.desc())
        .load::<Post>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nSELECT * FROM posts WHERE id = ?");
    let posts_result = posts
        .filter(id.eq(2))
        .load::<Post>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nUPDATE posts SET title = ? WHERE title = ?");
    diesel::update(posts.filter(title.eq("Nuevo")))
        .set(title.eq("My three post"))
        .get_result::<Post>(&mut conn)
        .expect("Error updated record");

    println!("\nSELECT * FROM posts WHERE title = ?");
    let posts_result = posts
        .filter(title.eq("My three post"))
        .load::<Post>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }

    println!("\nDELETE FROM posts WHERE title = ?");
    diesel::delete(posts.filter(title.eq("My three post")))
        .execute(&mut conn)
        .expect("Error delete record");

    println!("\nSELECT * FROM posts WHERE title = ?");
    let posts_result = posts
        .filter(title.eq("My three post"))
        .load::<Post>(&mut conn)
        .expect("Error execute query");
    for post in posts_result {
        println!("- {:?}", post);
    }
}
