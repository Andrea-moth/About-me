use about_me::Charity;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Scope,
};
use log::error;
use sqlx::{Pool, Sqlite};

pub fn append_routes() -> Scope {
    web::scope("/charities")
        .service(retrieve_charities)
        .service(retrieve_one_charity)
        .service(update_charity)
        .service(create_charity)
        .service(delete_charity)
}

#[get("")]
pub async fn retrieve_charities(database: web::Data<Pool<Sqlite>>) -> HttpResponse {
    if let Ok(charity) = sqlx::query_as!(
        Charity,
        r#"SELECT short, flavor_text, link_text, link FROM charities"#
    )
    .fetch_one(&**database)
    .await
    {
        HttpResponse::Ok().json(charity)
    } else {
        HttpResponse::NoContent().finish()
    }
}

/// Gets a specific project
///
/// Takes a short and returns the project with that short
#[get("/{short}")]
async fn retrieve_one_charity(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let short = short.into_inner();
    if let Ok(project) = sqlx::query_as!(
        Charity,
        r#"SELECT short, flavor_text, link_text, link FROM charities WHERE short = ?"#,
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
async fn update_charity(
    short: web::Path<String>,
    Json(button): Json<Charity>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = (&*database).acquire().await else {
        return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish()
    };
    let short = short.into_inner();

    if let Err(err) = sqlx::query!(
        r#"UPDATE charities SET flavor_text = ?, link_text = ?, link = ? WHERE short = ?"#,
        button.flavor_text,
        button.link_text,
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
async fn create_charity(button: Json<Charity>, database: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let Ok(mut conn) = (&*database)
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let button = button.into_inner();

    if let Err(err) = sqlx::query!(
        r#"INSERT INTO charities( short, flavor_text, link_text, link ) VALUES ( ?1, ?2, ?3, ?4)"#,
        button.short,
        button.flavor_text,
        button.link_text,
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
async fn delete_charity(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = (&*database)
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let short = short.into_inner();

    if let Err(why) = sqlx::query!("DELETE FROM charities WHERE short = ?", short)
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
