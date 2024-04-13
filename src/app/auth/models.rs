use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserPasswordCredentials {
    pub username: String,
    pub password: String,
}
