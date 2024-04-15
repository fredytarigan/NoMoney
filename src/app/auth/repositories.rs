use super::models::UserPasswordCredentials;
use crate::Response;
use crate::{app::users::LoginUser, errors::ApplicationError, redis::CacheConn};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use mobc_redis::redis::AsyncCommands;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;

pub struct Repository;

impl Repository {
    pub async fn authorize_user(
        user: &LoginUser,
        credentials: UserPasswordCredentials,
    ) -> Result<String, ApplicationError> {
        let argon = Argon2::default();
        let db_hash = PasswordHash::new(&user.password)?;

        argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

        // generate token for session
        let token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        Ok(token)
    }

    pub async fn set_session_cache(
        cache: &mut CacheConn,
        path: String,
        value: String,
        ttl: usize,
    ) -> Result<(), ApplicationError> {
        let _ = cache
            .set_ex::<String, String, ()>(path, value, ttl)
            .await
            .map_err(|e| {
                error!("Session Set Error: {}", e.to_string());
                let response = Response::new(
                    500,
                    5000,
                    String::from("cache connection error"),
                    None,
                    Some(json!("cache error")),
                );

                ApplicationError::new(response)
            });

        Ok(())
    }
}
