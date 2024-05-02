use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;

use crate::utils::errors::ApiError;

pub fn hash_password(password: String) -> Result<String, ApiError> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();

    let hashed_password = argon.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}
