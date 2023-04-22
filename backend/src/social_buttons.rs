use about_me::SocialLink;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Scope,
};
use log::error;
use sqlx::{Pool, Sqlite};

pub fn append_routes() -> Scope {
    web::scope("/social_buttons")
        .service(retrieve_social_buttons)
        .service(retrieve_one_button)
        .service(update_button)
        .service(create_button)
        .service(delete_button)
}

#[get("")]
pub async fn retrieve_social_buttons(database: web::Data<Pool<Sqlite>>) -> HttpResponse {
    if let Ok(social_buttons) = sqlx::query_as!(
        SocialLink,
        r#"SELECT short, at, name, link FROM social_buttons"#
    )
    .fetch_all(&**database.clone())
    .await
    {
        HttpResponse::Ok().json(social_buttons)
    } else {
        HttpResponse::NoContent().finish()
    }
}

/// Gets a specific project
///
/// Takes a short and returns the project with that short
#[get("/{short}")]
async fn retrieve_one_button(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let short = short.into_inner();
    if let Ok(project) = sqlx::query_as!(
        SocialLink,
        r#"SELECT short, at, name, link FROM social_buttons WHERE link = ?"#,
        short
    )
    .fetch_one(&**database.clone())
    .await
    {
        HttpResponse::Ok().json(project)
    } else {
        error!("Couldn't find project {short}");
        HttpResponse::NotFound()
            .reason("Cannot find the project with that name")
            .finish()
    }
}

/// Updates a project
///
/// Takes a short that is used to reference the row to be updated and a json form of a project and
/// updates the database with it
#[put("/{short}")]
async fn update_button(
    short: web::Path<String>,
    button: Json<SocialLink>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = (&*database).acquire().await else {
        return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish()
    };
    let button = button.into_inner();
    let short = short.into_inner();

    if let Err(err) = sqlx::query!(
        r#"UPDATE social_buttons SET at = ?, name = ?, link = ? WHERE short = ?"#,
        button.at,
        button.name,
        button.link,
        short
    )
    .execute(&mut conn)
    .await
    {
        error!("Error with database, insert invalid {err}");
        HttpResponse::ServiceUnavailable()
            .reason("Could not update database")
            .finish()
    } else {
        HttpResponse::Ok().json(button)
    }
}

/// Adds a project
///
/// Takes a json form of a project and creates a row in the projects table with that data
#[post("")]
async fn create_button(
    button: Json<SocialLink>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = (&*database)
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let button = button.into_inner();

    if let Err(err) = sqlx::query!(
        r#"INSERT INTO social_buttons( short, at, name, link ) VALUES ( ?1, ?2, ?3, ?4)"#,
        button.short,
        button.at,
        button.name,
        button.link,
    )
    .execute(&mut conn)
    .await
    {
        error!("Error with database, insert invalid {err}");
        HttpResponse::ServiceUnavailable()
            .reason("Could not insert into database")
            .finish()
    } else {
        HttpResponse::Ok().json(button)
    }
}

/// Deletes a project
///
/// Takes a short that is used to reference the project and deletes the row with that short
#[delete("/{short}")]
async fn delete_button(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = (&*database)
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let short = short.into_inner();

    if let Err(why) = sqlx::query!("DELETE FROM social_buttons WHERE short = ?", short)
        .execute(&mut conn)
        .await
    {
        error!("Couldn't delete project with short: {short}, why: {why}");
        HttpResponse::ServiceUnavailable()
            .reason("Cannot find project in database")
            .finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
