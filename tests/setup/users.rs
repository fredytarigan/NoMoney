use reqwest::{blocking::Client, StatusCode};
use serde_json::json;
use serde_json::Value;

use super::APP_HOST;

pub fn create_test_users(client: &Client, family_id: &String, role: String) -> Value {
    // get role
    let roles = client
        .get(format!("{}/roles/search?name={}", APP_HOST, role))
        .send()
        .unwrap();

    let result: Value = roles.json().unwrap();
    let role_id = result["data"]["id"].as_str().unwrap();

    let response = client
        .post(format!("{}/users", APP_HOST))
        .json(&json!(
            {
                "username": format!("test.user.{}", role),
                "password": "12345678",
                "active": true,
                "family_id": family_id,
                "role_id": role_id,
                "email": format!("test.user.{}@example.org", role),
                "email_validated": true,
                "first_name": "Test",
                "last_name": format!("{}", role)
            }
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let users: Value = response.json().unwrap();

    assert_eq!(
        users["data"],
        json!(
            {
                "id": users["data"]["id"],
                "username": format!("test.user.{}", role),
                "active": true,
                "family_id": family_id,
                "role_id": role_id,
                "email": format!("test.user.{}@example.org", role),
                "email_validated": true,
                "first_name": "Test",
                "last_name": format!("{}", role),
                "created_at": users["data"]["created_at"],
                "updated_at": users["data"]["updated_at"],
            }
        )
    );

    return users;
}

pub fn delete_test_users(client: &Client, users: Value) {
    let uid = users["data"]["id"].as_str().unwrap();

    let response = client
        .delete(format!("{}/users/{}", APP_HOST, uid))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
