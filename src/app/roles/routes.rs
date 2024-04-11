use super::repositories::RolesRepository;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    get,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;
use uuid::Uuid;

pub struct RouteRoles;

impl RouteRoles {
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

    let roles = RolesRepository::find_all(&mut conn, 100).await?;

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
    let uid = match Uuid::parse_str(&role_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError::new(
                422,
                format!("invalid input for uid with value: {}", role_id.to_string()),
            ))
        }
    };

    let mut conn = db.get().await?;

    let roles = RolesRepository::find_by_id(&mut conn, uid).await?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": roles,
            "message": null,
        }
    )))
}
