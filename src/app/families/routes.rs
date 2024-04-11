use super::models::*;
use super::repositories::FamilyRepository;
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
use uuid::Uuid;

pub struct RouteFamily;

impl RouteFamily {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/families")
                .service(index_family)
                .service(view_family)
                .service(create_family)
                .service(update_family)
                .service(delete_family),
        );
    }
}

#[get("")]
async fn index_family(db: web::Data<DbPool>) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let families = FamilyRepository::find_all(&mut conn, 100).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[get("/{family_id}")]
async fn view_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError::new(
                422,
                format!(
                    "invalid input for uid with value: {}",
                    family_id.to_string()
                ),
            ))
        }
    };

    let mut conn = db.get().await?;

    let families = FamilyRepository::find_by_id(&mut conn, uid).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[post("")]
async fn create_family(
    db: web::Data<DbPool>,
    data: web::Json<NewFamily>,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let families = FamilyRepository::create(&mut conn, data.into_inner()).await?;

    Ok(HttpResponse::Created().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[put("/{family_id}")]
async fn update_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    data: web::Json<Family>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError::new(
                422,
                format!(
                    "invalid input for uid with value: {}",
                    family_id.to_string()
                ),
            ))
        }
    };

    let mut conn = db.get().await?;

    let families = FamilyRepository::update(&mut conn, uid, data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": families,
            "message": null,
        }
    )))
}

#[delete("/{family_id}")]
async fn delete_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError::new(
                422,
                format!(
                    "invalid input for uid with value: {}",
                    family_id.to_string()
                ),
            ))
        }
    };

    let mut conn = db.get().await?;

    let _ = FamilyRepository::delete(&mut conn, uid).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
