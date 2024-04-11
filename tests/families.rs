pub mod setup;

use reqwest::StatusCode;
use serde_json::{json, Value};
use setup::{setup_client, APP_HOST};

use crate::setup::families::delete_test_families;

#[test]
fn test_get_families() {
    // Setup
    let client = setup_client();
    let families = setup::families::create_test_families(&client);

    // Test
    let response = client.get(format!("{}/families", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json["data"].as_array().unwrap().contains(&families["data"]));

    // Cleanup
    delete_test_families(&client, families);
}

#[test]
fn test_create_families() {
    // Setup
    let client = setup_client();
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

    // Cleanup
    delete_test_families(&client, families);
}

