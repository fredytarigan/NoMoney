use super::models::*;
use super::repositories::FamiliesRepository;
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

pub struct RouteFamilies;

impl RouteFamilies {
    pub fn init(cfg: &mut web::ServiceConfig) {
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

    let families = FamiliesRepository::find_all(&mut conn, 100).await?;

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

    let families = FamiliesRepository::find_by_id(&mut conn, uid).await?;

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
    data: web::Json<NewFamily>,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let families = FamiliesRepository::create(&mut conn, data.into_inner()).await?;

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

    let families = FamiliesRepository::update(&mut conn, uid, data.into_inner()).await?;

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

    let _ = FamiliesRepository::delete(&mut conn, uid).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}
