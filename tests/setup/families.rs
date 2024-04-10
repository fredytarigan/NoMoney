use reqwest::{blocking::Client, StatusCode};
use serde_json::json;
use serde_json::Value;

use super::APP_HOST;

pub fn create_test_families(client: &Client) -> Value {
    let response = client
        .post(format!("{}/families", APP_HOST))
        .json(&json!(
            {
                "name": "Test Family",
                "description": "Rust Test Family Data"
            }
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let json: Value = response.json().unwrap();

    assert_eq!(
        json["data"],
        json!({
            "id": json["data"]["id"],
            "name": "Test Family",
            "description": "Rust Test Family Data",
            "created_at": json["data"]["created_at"],
            "updated_at": json["data"]["updated_at"],
        })
    );

    return json;
}

pub fn delete_test_families(client: &Client, families: Value) {
    let uid = families["data"]["id"].as_str().unwrap();

    let response = client
        .delete(format!("{}/families/{}", APP_HOST, uid))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
