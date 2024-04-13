use crate::schema::roles;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct CreateRole {
    pub name: String,
    pub description: String,
}
