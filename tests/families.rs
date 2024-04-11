pub mod setup;

use reqwest::StatusCode;
use serde_json::{json, Value};
use setup::{families::create_test_families, setup_client, APP_HOST};

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

    // Test
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

#[test]
fn test_view_families() {
    /* Setup Section */
    let client = setup_client();
    let families = create_test_families(&client);
    let uid = families["data"]["id"].as_str().unwrap();

    /* Test Section */
    let response = client
        .get(format!("{}/families/{}", APP_HOST, uid))
        .send()
        .unwrap();

    // response test
    // should be 200
    assert_eq!(response.status(), StatusCode::OK);

    let families: Value = response.json().unwrap();

    // data test
    // left side must matched right side
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

    // test response code when invalid ID is provided
    // should return 404
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .get(format!("{}/families/{}", APP_HOST, dummy_uuid))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    /* Cleanup Section */
    delete_test_families(&client, families);
}

#[test]
fn test_update_families() {
    /* Setup Section */
    let client = setup_client();
    let families = create_test_families(&client);
    let uid = families["data"]["id"].as_str().unwrap();

    /* Test Section */
    let response = client
        .put(format!("{}/families/{}", APP_HOST, uid))
        .json(&json!({
            "name": "Modified Family Name",
            "description": "Modified Family Description"
        }))
        .send()
        .unwrap();

    // response test
    // should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    // left side should matched right side
    let families: Value = response.json().unwrap();
    assert_eq!(
        families["data"],
        json!({
            "id": families["data"]["id"],
            "name": "Modified Family Name",
            "description": "Modified Family Description",
            "created_at": families["data"]["created_at"],
            "updated_at": families["data"]["updated_at"],
        })
    );

    // try update data with dummy id
    // test should be failed with 404
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .put(format!("{}/families/{}", APP_HOST, dummy_uuid))
        .json(&json!({
            "name": "Modified Family Name",
            "description": "Modified Family Description"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    /* Cleanup Section */
    delete_test_families(&client, families);
}
