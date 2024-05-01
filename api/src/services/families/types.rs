use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

/* base struct for routes object */
pub struct Routes;

#[derive(Serialize)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
