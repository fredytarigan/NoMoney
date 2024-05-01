use reqwest::{blocking::Client, StatusCode};
use serde_json::json;
use serde_json::Value;

use super::APP_HOST;

pub fn create_test_families(client: &Client) -> Value {
    let response = client
        .post(format!("{}/families", APP_HOST))
        .json(&json!(
            {
                "name": "Test Family Name",
                "description": "Test Family Description"
            }
        ))
        .send()
        .unwrap();

    eprintln!("{:?}", response);

    assert_eq!(response.status(), StatusCode::CREATED);

    let families: Value = response.json().unwrap();

    assert_eq!(
        families["data"],
        json!({
            "id": families["data"]["id"],
            "name": "Test Family Name",
            "description": "Test Family Description",
            "created_at": families["data"]["created_at"],
            "updated_at": families["data"]["updated_at"],
        })
    );

    return families;
}

pub fn delete_test_families(client: &Client, families: Value) {
    let uid = families["data"]["id"].as_str().unwrap();

    let response = client
        .delete(format!("{}/families/{}", APP_HOST, uid))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
