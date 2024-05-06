use std::time::SystemTime;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, Header};
use uuid::Uuid;

/* internal dependency */
use super::{
    helpers,
    models::{LoginUser, UserIsActive, UserPasswordAuth, UserRole},
};
use crate::{
    common::{configs::Config, databases::DbPool},
    services::auth::types::{AuthError, Claims, Keys},
    utils::errors::ApiError,
};

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
                user_id,
                username,
                password,
                email,
                email_verified,
                active,
                role_id
            FROM
                users
            WHERE
                username = $1
            ",
            user.username
        )
        .fetch_one(db)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

        Ok(user)
    }

    pub async fn get_role_by_username(db: &DbPool, user: &LoginUser) -> Result<UserRole, ApiError> {
        let role = sqlx::query_as!(
            UserRole,
            "
            SELECT
                role_id,
                role_name
            FROM
                roles
            WHERE
                role_id = $1
            ",
            user.role_id
        )
        .fetch_one(db)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

        Ok(role)
    }

    pub async fn authorized_user(
        config: &Config,
        user: &LoginUser,
        role: &UserRole,
        credentials: &UserPasswordAuth,
    ) -> Result<String, ApiError> {
        let argon = Argon2::default();

        let data = helpers::hash_password(String::from("admin"));
        println!("{:?}", data);
        let password_hash = PasswordHash::new(&user.password)?;

        argon
            .verify_password(credentials.password.as_bytes(), &password_hash)
            .map_err(|_| AuthError::WrongCredentials)?;

        // set jwt token expiry time
        // expiry time will be set from configuration
        let exp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + config.jwt.duration;

        let iat = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            aud: None,
            iat: usize::try_from(iat).unwrap(),
            iss: String::from("Octopus API"),
            nbf: usize::try_from(exp).unwrap(),
            sub: user.user_id,
            username: user.username.to_owned(),
            exp: usize::try_from(exp).unwrap(),
            auth_time: chrono::Utc::now().naive_utc(),
            email: user.email.to_owned(),
            email_verified: user.email_verified,
            active: user.active,
            role: role.role_name.to_owned(),
        };

        let keys = Keys::new(config.jwt.secret.as_bytes());

        // generate random token if login success
        let token = encode(&Header::default(), &claims, &keys.encoding)
            .map_err(|_| AuthError::TokenCreation)?;

        Ok(token)
    }

    pub async fn check_user_is_active(db: &DbPool, user_id: &Uuid, username: &String) -> bool {
        let users = sqlx::query_as!(
            UserIsActive,
            "
            SELECT
                user_id,
                username,
                active
            FROM
                users
            WHERE
                user_id = $1 AND username = $2 AND active = TRUE
            ",
            user_id,
            username
        )
        .fetch_one(db)
        .await
        .map_err(|_| false);

        if let Ok(user) = users {
            if user.username == "" {
                return false;
            } else {
                return true;
            }
        };

        false
    }
}
