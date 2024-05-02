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
}
