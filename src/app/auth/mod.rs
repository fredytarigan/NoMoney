mod models;
mod repositories;
mod routes;

pub use routes::Router;

use super::users::LoggedUser;
use crate::app::users::Repository as UserRepository;
use crate::redis::CachePool;
use crate::{database::DbPool, errors::ApplicationError};

use actix_web::{web, FromRequest};
use mobc_redis::redis::AsyncCommands;

impl FromRequest for LoggedUser {
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
                    Err(ApplicationError::new(403, String::from("Unauthorized")))
                });
            }
            Some(header) => header[1].to_string(),
        };

        info!("{:?}", token_value);

        let db = match req.app_data::<web::Data<DbPool>>() {
            Some(db) => db.to_owned(),
            None => {
                error!("Database Connection Error");
                return Box::pin(async move {
                    Err(ApplicationError::new(
                        500,
                        String::from("Unhandled error happen at the server"),
                    ))
                });
            }
        };

        let cache = match req.app_data::<web::Data<CachePool>>() {
            Some(cache) => cache.to_owned(),
            None => {
                error!("Cache Connection Error");
                return Box::pin(async move {
                    Err(ApplicationError::new(
                        500,
                        String::from("Unhandled error happen at the server"),
                    ))
                });
            }
        };

        // let bind_db = db.to_owned();

        Box::pin(async move {
            let mut conn = db.get().await?;
            let mut cache = cache.get().await.unwrap();

            let result = cache
                .get::<String, String>(format!("nomoney/session/{}", token_value))
                .await;

            if let Ok(user_id) = result {
                let id = uuid::Uuid::parse_str(&user_id).unwrap();
                if let Ok(user) = UserRepository::find_by_id(&mut conn, id).await {
                    return Ok(LoggedUser {
                        id: user.id,
                        username: user.username,
                    });
                }
            }

            Err(ApplicationError::new(403, String::from("Unauthorized")))
        })
    }
}
