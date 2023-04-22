//mod auth;
mod charity;
mod error;
mod projects;
mod social_buttons;

use std::env::current_dir;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use error::ServerResult;
use sqlx::SqlitePool;

use log::info;

#[cfg(debug_assertions)]
const DIST_PATH: &str = "../dist";

#[cfg(not(debug_assertions))]
const DIST_PATH: &str = "./dist";

#[actix_web::main]
async fn main() -> ServerResult<()> {
    println!("{:?}", current_dir());
    env_logger::init();
    let pool = SqlitePool::connect("sqlite://projects.db").await?;
    info!("Connected to database!");

    info!("Building server!");
    HttpServer::new(move || {
        App::new()
            .service(projects::append_routes())
            .service(charity::append_routes())
            .service(social_buttons::append_routes())
            .service(Files::new("/", DIST_PATH).index_file("index.html"))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8888))
    .unwrap()
    .run()
    .await?;
    Ok(())
}
