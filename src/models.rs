use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Family {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=family)]
pub struct NewFamily {
    pub name: String,
    pub description: String,
}
