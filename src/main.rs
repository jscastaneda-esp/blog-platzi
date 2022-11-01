mod models;
mod schema;

use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenvy::dotenv;
use models::{NewPostHandler, Post};
use schema::posts::dsl::posts;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let mut conn = pool.get().expect("Error getting connection from database");

    match web::block(move || posts.load::<Post>(&mut conn).unwrap()).await {
        Ok(data) => Ok(web::Json(data)),
        Err(_) => Err(ErrorInternalServerError("Error")),
    }
}

#[post("/new_post")]
async fn new_post(
    pool: web::Data<DbPool>,
    item: web::Json<NewPostHandler>,
) -> Result<impl Responder> {
    let mut conn = pool.get().expect("Error getting connection from database");

    match web::block(move || Post::create_post(&mut conn, &item).unwrap()).await {
        Ok(data) => Ok(web::Json(data)),
        Err(_) => Err(ErrorInternalServerError("Error")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DB Url config not found");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error get pool connections database");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(new_post)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 9900))?
    .run()
    .await
}
