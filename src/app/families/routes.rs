use super::models::*;
use super::repositories::Repository;
use crate::app::utils::parse_uuid;
use crate::app::RouterConfig;
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
async fn index_families(db: web::Data<DbPool>) -> Result<HttpResponse, ApplicationError> {
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
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&family_id)?;

    let mut conn = db.get().await?;

    let families = Repository::find_by_id(&mut conn, uid).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[post("")]
async fn create_families(
    db: web::Data<DbPool>,
    data: web::Json<CreateFamily>,
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
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&family_id)?;

    let mut conn = db.get().await?;

    let _ = Repository::delete(&mut conn, uid).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
