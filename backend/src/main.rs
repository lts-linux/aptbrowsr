use actix_web::{web, App, HttpServer};
use std::io;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2;
use diesel::SqliteConnection;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

mod distros;

use crate::distros::{list_distros, new_distro};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to SQLite DB
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_distros)
            .service(new_distro)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
