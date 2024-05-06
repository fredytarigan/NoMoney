use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserPasswordAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_verified: bool,
    pub active: bool,
    pub role_id: Uuid,
}

#[derive(Deserialize)]
pub struct UserRole {
    pub role_id: Uuid,
    pub role_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserIsActive {
    pub user_id: Uuid,
    pub username: String,
    pub active: bool,
}
