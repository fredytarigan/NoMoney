use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name=families)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=families)]
pub struct NewFamily {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
