use super::models::*;
use super::repositories::Repository;
use crate::app::permissions::EditorUser;
use crate::app::roles::Repository as RolesRepository;
use crate::app::{permissions::AdminUser, utils::parse_uuid};
use crate::app::{Response, RouterConfig};
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

impl RouterConfig for Router {
    fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/families")
                .service(index_families)
                .service(view_families)
                .service(create_families)
                .service(update_families)
                .service(delete_families),
        );
    }
}

#[get("")]
async fn index_families(
    db: web::Data<DbPool>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let families = Repository::find_all(&mut conn, 100).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[get("/{family_id}")]
async fn view_families(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&family_id)?;

    let mut conn = db.get().await?;

    let roles = RolesRepository::find_by_id(&mut conn, user.role_id).await?;

    if roles.name == "admin" || uid == user.family_id {
        let families = Repository::find_by_id(&mut conn, uid).await?;

        Ok(HttpResponse::Ok().json(json!(
            {
                "status": "success",
                "data": families,
                "message": null,
            }
        )))
    } else {
        let response = Response::new(
            403,
            4003,
            String::from("unauthorized request"),
            None,
            Some(json!(["unauthorized"])),
        );

        Err(ApplicationError::new(response))
    }
}

#[post("")]
async fn create_families(
    db: web::Data<DbPool>,
    data: web::Json<CreateFamily>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let families = Repository::create(&mut conn, data.into_inner()).await?;

    Ok(HttpResponse::Created().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[put("/{family_id}")]
async fn update_families(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    data: web::Json<Family>,
    _user: EditorUser,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&family_id)?;

    let mut conn = db.get().await?;

    let families = Repository::update(&mut conn, uid, data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[delete("/{family_id}")]
async fn delete_families(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&family_id)?;

    let mut conn = db.get().await?;

    let _ = Repository::delete(&mut conn, uid).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
