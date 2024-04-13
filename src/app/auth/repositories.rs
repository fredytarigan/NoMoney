use super::models::UserPasswordCredentials;
use crate::{app::users::LoginUser, errors::ApplicationError};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::{distributions::Alphanumeric, Rng};

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
}
