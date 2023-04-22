use about_me::Project;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Scope,
};
use log::{error, info};
use sqlx::{Pool, Sqlite};

pub fn append_routes() -> Scope {
    web::scope("/projects")
        .service(retrieve_projects)
        .service(retrieve_one_project)
        .service(update_project)
        .service(create_project)
        .service(delete_project)
}

/// Gets all projects
///
/// Retrieves all the projects in the database newest first
#[get("")]
async fn retrieve_projects(database: web::Data<Pool<Sqlite>>) -> HttpResponse {
    if let Ok(mut projects) = sqlx::query_as!(
        Project,
        r#"SELECT short, alt, name, image_path, description FROM projects"#
    )
    .fetch_all(&**database.clone())
    .await
    {
        projects.reverse();
        HttpResponse::Ok().json(projects)
    } else {
        HttpResponse::NoContent().finish()
    }
}

/// Gets a specific project
///
/// Takes a short and returns the project with that short
#[get("/{short}")]
async fn retrieve_one_project(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let short = short.into_inner();
    if let Ok(project) = sqlx::query_as!(
        Project,
        r#"SELECT short, alt, name, image_path, description FROM projects WHERE short = ?"#,
        short
    )
    .fetch_one(&**database.clone())
    .await
    {
        HttpResponse::Ok().json(project)
    } else {
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
async fn update_project(
    short: web::Path<String>,
    project: Json<Project>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = database.acquire().await else {
        return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish()
    };
    let project = project.into_inner();
    let short = short.into_inner();

    if let Err(_) = sqlx::query!(
        r#"UPDATE projects SET alt = ?, name = ?, image_path = ?, description = ? WHERE short = ?"#,
        project.alt,
        project.name,
        project.image_path,
        project.description,
        short
    )
    .execute(&mut conn)
    .await
    {
        HttpResponse::ServiceUnavailable()
            .reason("Could not update database")
            .finish()
    } else {
        HttpResponse::Ok().json(project)
    }
}

/// Adds a project
///
/// Takes a json form of a project and creates a row in the projects table with that data
#[post("")]
async fn create_project(project: Json<Project>, database: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let Ok(mut conn) = database
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let project = project.into_inner();

    if let Err(err) = sqlx::query!(
        r#"INSERT INTO projects( short, alt, name, image_path, description ) VALUES ( ?1, ?2, ?3, ?4, ?5)"#,
        project.short,
        project.alt,
        project.name,
        project.image_path,
        project.description,
    )
    .execute(&mut conn)
    .await
    {
        error!("Error with database, insert invalid {err}");
        HttpResponse::ServiceUnavailable()
            .reason("Could not insert into database")
            .finish()
    } else {
        HttpResponse::Ok().json(project)
    }
}

/// Deletes a project
///
/// Takes a short that is used to reference the project and deletes the row with that short
#[delete("/{short}")]
pub async fn delete_project(
    short: web::Path<String>,
    database: web::Data<Pool<Sqlite>>,
) -> HttpResponse {
    let Ok(mut conn) = database
        .acquire()
        .await else {
            return HttpResponse::ServiceUnavailable().reason("Database unavailable").finish();
        };
    let short = short.into_inner();

    if let Err(why) = sqlx::query!("DELETE FROM projects WHERE short = ?", short)
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
