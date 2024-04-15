use super::models::*;
use super::repositories::Repository;
// use crate::app::utils::parse_uuid;
use crate::app::users::Repository as UserRepository;
use crate::app::RouterConfig;
use crate::errors::ApplicationError;
use crate::Response;
use crate::{database::DbPool, redis::CachePool};
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
    cache: web::Data<CachePool>,
    credentials: web::Json<UserPasswordCredentials>,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;
    let user = UserRepository::login_by_username(&mut conn, &credentials.username)
        .await
        .map_err(|e| {
            error!("Login Error: {}", e);
            let response = Response::new(
                500,
                5000,
                String::from("cache connection error"),
                None,
                Some(json!("cache error")),
            );

            ApplicationError::new(response)
        })?;

    let token = Repository::authorize_user(&user, credentials.into_inner())
        .await
        .map_err(|e| {
            info!("Authorized Error: {}", e);

            let response = Response::new(
                403,
                4003,
                String::from("unauthorized request"),
                None,
                Some(json!(["unauthorized"])),
            );

            ApplicationError::new(response)
        })?;

    let mut cache_conn = cache.get().await.unwrap();

    // construct session in cache server
    let session_path = format!("nomoney/session/{}", token);
    let session_value = format!("{}", user.id);
    let session_ttl = 3 * 60 * 60;

    Repository::set_session_cache(&mut cache_conn, session_path, session_value, session_ttl)
        .await?;

    Ok(Response::new(
        200,
        2000,
        String::from("login success"),
        Some(json!({
            "token": token
        })),
        None,
    )
    .return_ok())
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
