use super::models::*;
use super::repositories::Repository;
// use crate::app::utils::parse_uuid;
use crate::app::users::Repository as UserRepository;
use crate::app::RouterConfig;
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    post,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;

pub struct Router;

impl RouterConfig for Router {
    fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auth")
                .service(login_user_password)
                .service(logout_user),
        );
    }
}

#[post("/login")]
async fn login_user_password(
    db: web::Data<DbPool>,
    credentials: web::Json<UserPasswordCredentials>,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let user = UserRepository::login_by_username(&mut conn, &credentials.username)
        .await
        .map_err(|e| {
            error!("Login Error: {}", e);
            ApplicationError::new(403, format!("Unauthorized, please check your credentials"))
        })?;

    let token = Repository::authorize_user(&user, credentials.into_inner())
        .await
        .map_err(|e| {
            info!("Authorized Error: {}", e);
            ApplicationError::new(403, format!("Unauthorized, please check your credentials"))
        })?;

    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": {
                "token": token,
            },
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
