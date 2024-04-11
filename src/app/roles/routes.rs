use super::models::*;
use super::repositories::RolesRepository;
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

pub struct RouteRoles;

impl RouteRoles {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/roles").service(index_roles));
    }
}

#[get("")]
async fn index_roles(db: web::Data<DbPool>) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let roles = RolesRepository::find_all(&mut conn, 100).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": roles,
        "message": null,
    })))
}
