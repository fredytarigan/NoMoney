use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name=families)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=families)]
pub struct NewFamily {
    pub name: String,
    pub description: String,
}
