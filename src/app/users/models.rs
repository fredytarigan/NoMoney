use crate::app::{families::Family, roles::Role};
use crate::schema::users;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations, Insertable)]
#[diesel(table_name = users)]
#[diesel(belongs_to(Family))]
#[diesel(belongs_to(Role))]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(skip_deserializing)]
    pub password: String,
    pub active: bool,
    pub family_id: Uuid,
    pub role_id: Uuid,
    pub email: String,
    pub email_validated: bool,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub active: bool,
    pub family_id: Uuid,
    pub role_id: Uuid,
    pub email: String,
    pub email_validated: bool,
}

#[derive(Queryable, Serialize, Deserialize, QueryableByName, Debug)]
#[diesel(table_name = users)]
pub struct GetUser {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: bool,
    pub family_id: Uuid,
    pub role_id: Uuid,
    pub email: String,
    pub email_validated: bool,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

impl From<User> for GetUser {
    fn from(user: User) -> Self {
        GetUser {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            active: user.active,
            family_id: user.family_id,
            role_id: user.role_id,
            email: user.email,
            email_validated: user.email_validated,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
