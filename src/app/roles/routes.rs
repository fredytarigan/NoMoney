use super::repositories::Repository;
use crate::app::utils::parse_uuid;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    get,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;

pub struct Router;

impl Router {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/roles")
                .service(index_roles)
                .service(view_roles),
        );
    }
}

#[get("")]
async fn index_roles(db: web::Data<DbPool>) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let roles = Repository::find_all(&mut conn, 100).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": roles,
        "message": null,
    })))
}

#[get("/{role_id}")]
async fn view_roles(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let role_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&role_id)?;

    let mut conn = db.get().await?;

    let roles = Repository::find_by_id(&mut conn, uid).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": roles,
            "message": null,
        }
    )))
}
