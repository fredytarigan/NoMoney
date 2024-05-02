use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;

/* internal dependency */
use super::{
    helpers,
    models::{LoginUser, UserPasswordAuth},
};
use crate::{common::databases::DbPool, utils::errors::ApiError};

pub struct Repository;

impl Repository {
    pub async fn login_by_username(
        db: &DbPool,
        user: &UserPasswordAuth,
    ) -> Result<LoginUser, ApiError> {
        let user = sqlx::query_as!(
            LoginUser,
            "
            SELECT
                id,
                username,
                password
            FROM
                users
            WHERE
                username = $1
            ",
            user.username
        )
        .fetch_one(db)
        .await
        .map_err(|_| ApiError {
            code: 401,
            message: String::from("Unauthorized"),
            errors: Some(json!("[unauthorized request]")),
            status: String::from("error"),
            ..ApiError::default()
        })?;

        Ok(user)
    }

    pub async fn authorized_user(
        user: &LoginUser,
        credentials: &UserPasswordAuth,
    ) -> Result<String, ApiError> {
        let argon = Argon2::default();

        let data = helpers::hash_password(String::from("admin"));
        println!("{:?}", data);
        let password_hash = PasswordHash::new(&user.password)?;

        argon
            .verify_password(credentials.password.as_bytes(), &password_hash)
            .map_err(|_| ApiError {
                code: 401,
                message: String::from("Unauthorized"),
                errors: Some(json!("[unauthorized request]")),
                status: String::from("error"),
                ..ApiError::default()
            })?;

        // generate random token if login success
        let token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        Ok(token)
    }
}
