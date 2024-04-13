use super::models::*;
// use super::repositories::Repository;
// use crate::app::utils::parse_uuid;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    post,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;

pub struct Router;

impl Router {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auth")
                .service(login_user_password)
                .service(logout_user),
        );
    }
}

#[post("/login")]
async fn login_user_password(
    _db: web::Data<DbPool>,
    _data: web::Json<UserPasswordCredentials>,
) -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": null,
            "message": null,
        }
    )))
}

#[post("/logout")]
async fn logout_user() -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": null,
            "message": null,
        }
    )))
}
