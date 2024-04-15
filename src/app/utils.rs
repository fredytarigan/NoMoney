use crate::{app::Response, errors::ApplicationError};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;
use serde_json::json;
use uuid::Uuid;

pub fn hash_password(password: String) -> Result<String, ApplicationError> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}

pub fn parse_uuid(uuid: &String) -> Result<Uuid, ApplicationError> {
    match Uuid::parse_str(uuid) {
        Ok(uid) => Ok(uid),
        Err(_) => {
            error!("Invalid input for user id with value: {}", uuid.to_string());

            let response = Response::new(
                400,
                4000,
                String::from("invalid input provided for resources"),
                None,
                Some(json!(["invalid input"])),
            );

            Err(ApplicationError::new(response))
        }
    }
}
