use super::models::*;
use super::repositories::Repository;
use crate::app::utils::parse_uuid;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    delete, get,
    http::StatusCode,
    post, put,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;

pub struct Router;

impl Router {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/users")
                .service(index_users)
                .service(view_users)
                .service(create_users)
                .service(update_users)
                .service(delete_users),
        );
    }
}

#[get("")]
async fn index_users(
    db: web::Data<DbPool>,
    _user: LoggedUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let users = Repository::find_all(&mut conn, 100).await?;

    Ok(HttpResponse::Ok().json(json!(
            {
            "status": "success",
            "data": users,
            "message": null,
        }
    )))
}

#[get("/{user_id}")]
async fn view_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    _user: LoggedUser,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let users = Repository::find_by_id(&mut conn, uid).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": users,
            "message": null,
        }
    )))
}

#[post("")]
async fn create_users(
    db: web::Data<DbPool>,
    data: web::Json<CreateUser>,
    _user: LoggedUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let users: GetUser = Repository::create(&mut conn, data.into_inner()).await?;

    Ok(HttpResponse::Created().json(json!(
        {
            "status": "success",
            "data": users,
            "message": null,
        }
    )))
}

#[put("/{user_id}")]
async fn update_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    data: web::Json<User>,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let users = Repository::update(&mut conn, uid, data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": users,
            "message": null,
        }
    )))
}

#[delete("/{user_id}")]
async fn delete_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let _ = Repository::delete(&mut conn, uid).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
