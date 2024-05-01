use actix_web::{web, FromRequest};
use mobc_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::app::roles::Repository as RolesRepository;
use crate::app::users::Repository as UsersRepository;
use crate::app::Response;
use crate::{database::DbPool, errors::ApplicationError, redis::CachePool};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub role_id: Uuid,
    pub family_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct EditorUser {
    pub id: Uuid,
    pub username: String,
    pub role_id: Uuid,
    pub family_id: Uuid,
}

impl FromRequest for AdminUser {
    type Error = ApplicationError;

    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, ApplicationError>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .map(|v| v.to_str().unwrap().split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer")
            .to_owned();

        let token_value: String = match auth_header {
            None => {
                return Box::pin(async move {
                    let response = Response::new(
                        403,
                        4003,
                        String::from("unauthorized request"),
                        None,
                        Some(json!(["unauthorized"])),
                    );

                    Err(ApplicationError::new(response))
                });
            }
            Some(header) => header[1].to_string(),
        };

        let db = match req.app_data::<web::Data<DbPool>>() {
            Some(db) => db.to_owned(),
            None => {
                error!("Database Connection Error");
                return Box::pin(async move {
                    let response = Response::new(
                        500,
                        5000,
                        String::from("database connection error"),
                        None,
                        Some(json!(["database error"])),
                    );

                    Err(ApplicationError::new(response))
                });
            }
        };

        let cache = match req.app_data::<web::Data<CachePool>>() {
            Some(cache) => cache.to_owned(),
            None => {
                error!("Cache Connection Error");
                return Box::pin(async move {
                    let response = Response::new(
                        500,
                        5000,
                        String::from("cache connection error"),
                        None,
                        Some(json!("cache error")),
                    );

                    Err(ApplicationError::new(response))
                });
            }
        };

        Box::pin(async move {
            let mut conn = db.get().await?;
            let mut cache = cache.get().await.unwrap();

            let result = cache
                .get::<String, String>(format!("nomoney/session/{}", token_value))
                .await;

            if let Ok(user_id) = result {
                let id = uuid::Uuid::parse_str(&user_id).unwrap();
                if let Ok(user) = UsersRepository::find_by_id(&mut conn, id).await {
                    /*
                        Get roles.name from user.role_id
                    */
                    let roles = RolesRepository::find_by_id(&mut conn, user.role_id).await?;

                    /*
                       We compare user.role_id into admin role_id
                    */
                    if roles.name == "admin" {
                        return Ok(AdminUser {
                            id: user.id,
                            username: user.username,
                            role_id: user.role_id,
                            family_id: user.family_id,
                        });
                    }
                }
            }

            let response = Response::new(
                403,
                4003,
                String::from("unauthorized request"),
                None,
                Some(json!(["unauthorized"])),
            );

            Err(ApplicationError::new(response))
        })
    }
}

impl FromRequest for EditorUser {
    type Error = ApplicationError;

    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, ApplicationError>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .map(|v| v.to_str().unwrap().split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer")
            .to_owned();

        let token_value: String = match auth_header {
            None => {
                return Box::pin(async move {
                    let response = Response::new(
                        403,
                        4003,
                        String::from("unauthorized request"),
                        None,
                        Some(json!(["unauthorized"])),
                    );

                    Err(ApplicationError::new(response))
                });
            }
            Some(header) => header[1].to_string(),
        };

        let db = match req.app_data::<web::Data<DbPool>>() {
            Some(db) => db.to_owned(),
            None => {
                error!("Database Connection Error");
                return Box::pin(async move {
                    let response = Response::new(
                        500,
                        5000,
                        String::from("database connection error"),
                        None,
                        Some(json!(["database error"])),
                    );

                    Err(ApplicationError::new(response))
                });
            }
        };

        let cache = match req.app_data::<web::Data<CachePool>>() {
            Some(cache) => cache.to_owned(),
            None => {
                error!("Cache Connection Error");
                return Box::pin(async move {
                    let response = Response::new(
                        500,
                        5000,
                        String::from("cache connection error"),
                        None,
                        Some(json!("cache error")),
                    );

                    Err(ApplicationError::new(response))
                });
            }
        };

        Box::pin(async move {
            let mut conn = db.get().await?;
            let mut cache = cache.get().await.unwrap();

            let result = cache
                .get::<String, String>(format!("nomoney/session/{}", token_value))
                .await;

            if let Ok(user_id) = result {
                let id = uuid::Uuid::parse_str(&user_id).unwrap();
                if let Ok(user) = UsersRepository::find_by_id(&mut conn, id).await {
                    /*
                        Get roles.name from user.role_id
                    */
                    let roles = RolesRepository::find_by_id(&mut conn, user.role_id).await?;

                    /*
                       We compare user.role_id into admin role_id
                    */
                    if roles.name == "admin" || roles.name == "editor" {
                        return Ok(EditorUser {
                            id: user.id,
                            username: user.username,
                            role_id: user.role_id,
                            family_id: user.family_id,
                        });
                    }
                }
            }

            let response = Response::new(
                403,
                4003,
                String::from("unauthorized request"),
                None,
                Some(json!(["unauthorized"])),
            );

            Err(ApplicationError::new(response))
        })
    }
}
