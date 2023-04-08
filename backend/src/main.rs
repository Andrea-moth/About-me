mod error;

use about_me::{Charity, Project, SocialLink};
use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use error::ServerResult;
use sqlx::{Pool, Sqlite, SqlitePool};

use log::info;

#[get("/projects")]
async fn projects(database: web::Data<Pool<Sqlite>>) -> impl Responder {
    web::Json(
        sqlx::query_as!(
            Project,
            r#"SELECT name, image_path, description FROM projects"#
        )
        .fetch_all(&**database.clone())
        .await
        .unwrap_or(vec![]),
    )
}

#[get("/social_buttons")]
async fn social_buttons(database: web::Data<Pool<Sqlite>>) -> impl Responder {
    web::Json(
        sqlx::query_as!(SocialLink, r#"SELECT at, name, link FROM social_buttons"#)
            .fetch_all(&**database.clone())
            .await
            .unwrap_or(vec![]),
    )
}

#[get("/charity")]
async fn charity(database: web::Data<Pool<Sqlite>>) -> impl Responder {
    if let Ok(charity) = sqlx::query_as!(
        Charity,
        r#"SELECT flavor_text, link_text, link FROM charities"#
    )
    .fetch_one(&**database)
    .await
    {
        web::Json(charity)
    } else {
        web::Json(Charity::error())
    }
}

#[actix_web::main]
async fn main() -> ServerResult<()> {
    env_logger::init();
    let pool = SqlitePool::connect("sqlite://projects.db").await?;
    info!("Connected to database!");

    info!("Building server!");
    HttpServer::new(move || {
        App::new()
            .service(projects)
            .service(charity)
            .service(social_buttons)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8888))
    .unwrap()
    .run()
    .await?;
    Ok(())
}
