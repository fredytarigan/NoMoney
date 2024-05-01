use chrono::Utc;
use uuid::Uuid;

use super::types::Family;

pub async fn list_families() -> Result<Vec<Family>, ()> {
    Ok(vec![
        Family {
            id: Uuid::new_v4(),
            name: String::from("Mock - First Family"),
            description: Some(String::from("Mock - Description of first family")),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
        Family {
            id: Uuid::new_v4(),
            name: String::from("Mock - Second Family"),
            description: Some(String::from("Mock - Description of second family")),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
        Family {
            id: Uuid::new_v4(),
            name: String::from("Mock - Third Family"),
            description: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    ])
}
