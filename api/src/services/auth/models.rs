use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserPasswordAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_verified: bool,
    pub active: bool,
    pub role_id: Uuid,
}

#[derive(Deserialize)]
pub struct UserRole {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserIsActive {
    pub id: Uuid,
    pub username: String,
    pub active: bool,
}
