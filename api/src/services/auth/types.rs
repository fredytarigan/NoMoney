use chrono::NaiveDateTime;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/* base struct for routes object */
pub struct Routes;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub aud: Option<String>,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub sub: Uuid,
    pub username: String,
    pub exp: usize,
    pub auth_time: NaiveDateTime,
    pub email: String,
    pub email_verified: bool,
    pub active: bool,
    pub role: String,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
