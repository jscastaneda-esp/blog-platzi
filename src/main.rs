mod models;
mod schema;

use actix_web::{error::ErrorInternalServerError, middleware::Logger, HttpResponse};
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenvy::dotenv;
use models::{NewPostHandler, Post};
use schema::posts::dsl::{posts, slug};
use std::env;
use tera::{Context, Tera};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>, tera: web::Data<Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connection from database");

    match web::block(move || posts.load::<Post>(&mut conn).unwrap()).await {
        Ok(data) => {
            let mut ctx = Context::new();
            ctx.insert("posts", &data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(tera.render("index.html", &ctx).unwrap())
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connection from database");

    match web::block(move || {
        posts
            .filter(slug.eq(blog_slug.into_inner()))
            .load::<Post>(&mut conn)
            .unwrap()
    })
    .await
    {
        Ok(data) => {
            if data.is_empty() {
                return HttpResponse::NotFound().finish();
            }

            let mut ctx = Context::new();
            ctx.insert("post", &data[0]);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(tera.render("post.html", &ctx).unwrap())
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
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

// env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("Port variable not found");

    println!("Starting server in port {}", port);
    HttpServer::new(move || {
        let database_url = env::var("DATABASE_URL").expect("DB Url config not found");
        println!("DATABASE_URL = {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
        println!("Create connection manager");

        // PgConnection::establish(&database_url).expect("Error");
        // println!("Establish connection manual");

        use std::time::Duration;

        let pool = Pool::builder()
            .connection_timeout(Duration::new(1, 0))
            .build(manager)
            .expect("Error get pool connections database");
        println!("Create pool");
        let tera = Tera::new("templates/**/*");

        println!("Listening server");
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(new_post)
            .service(get_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.unwrap()))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
