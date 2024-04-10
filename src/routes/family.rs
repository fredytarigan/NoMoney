use crate::{repositories::FamilyRepository, routes::DbPool};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Result,
};
use serde_json::json;

pub struct RouteFamily;

impl RouteFamily {
    pub fn route(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/family")
                .service(index)
                .service(view_family)
                .service(create_family)
                .service(update_family)
                .service(delete_family),
        );
    }
}

#[get("")]
async fn index(mut db: web::Data<DbPool>) -> Result<impl Responder> {
    FamilyRepository::find_all(db, 100).await;
    Ok(Json(json!(
        {
            "result": "ok",
            "message": "Get ALL Family"
        }
    )))
}

#[get("/{family_id}")]
async fn view_family() -> Result<impl Responder> {
    Ok(Json(json!(
        {
            "result": "ok",
            "message": "Get Family By ID"
        }
    )))
}

#[post("")]
async fn create_family() -> Result<impl Responder> {
    Ok(Json(json!(
        {
            "result": "ok",
            "message": "Create Family"
        }
    )))
}

#[put("/{family_id}")]
async fn update_family() -> Result<impl Responder> {
    Ok(Json(json!(
        {
            "result": "ok",
            "message": "Update Family By ID"
        }
    )))
}

#[delete("/{family_id}")]
async fn delete_family() -> Result<impl Responder> {
    Ok(Json(json!({
        "result": "ok",
        "message": "Delete Family By ID"
    })))
}
