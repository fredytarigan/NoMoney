use super::models::*;
use super::repositories::Repository;
use crate::app::permissions::EditorUser;
use crate::app::utils::parse_uuid;
use crate::app::Response;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
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
        )
        .service(web::scope("/profile").service(get_profile));
    }
}

#[get("")]
async fn index_users(
    db: web::Data<DbPool>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let users = Repository::find_all(&mut conn, 100).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("list of users"),
        Some(json!(users)),
        None,
    )
    .return_ok())
}

#[get("/{user_id}")]
async fn view_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let users = Repository::find_by_id(&mut conn, uid).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("get user by id"),
        Some(json!(users)),
        None,
    )
    .return_ok())
}

#[post("")]
async fn create_users(
    db: web::Data<DbPool>,
    data: web::Json<CreateUser>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let users: GetUser = Repository::create(&mut conn, data.into_inner()).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("create user"),
        Some(json!(users)),
        None,
    )
    .return_ok())
}

#[put("/{user_id}")]
async fn update_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    data: web::Json<User>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let users = Repository::update(&mut conn, uid, data.into_inner()).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("update user"),
        Some(json!(users)),
        None,
    )
    .return_ok())
}

#[delete("/{user_id}")]
async fn delete_users(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&user_id)?;

    let mut conn = db.get().await?;
    let _ = Repository::delete(&mut conn, uid).await?;

    Ok(Response::new(200, 2000, String::from("delete user"), None, None).return_ok())
}

#[get("")]
async fn get_profile(
    db: web::Data<DbPool>,
    user: LoggedUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let profile = Repository::get_profile(&mut conn, user).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("get profile"),
        Some(json!(profile)),
        None,
    )
    .return_ok())
}
