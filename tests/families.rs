pub mod setup;

use reqwest::StatusCode;
use serde_json::{json, Value};
use setup::{families::create_test_families, setup_client, APP_HOST};

use crate::setup::families::delete_test_families;

#[test]
fn test_get_families() {
    /*
        Setup Section
    */
    let client = setup_client();
    let families = setup::families::create_test_families(&client);

    /*
       Test Section
    */
    // response test
    let response = client.get(format!("{}/families", APP_HOST)).send().unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let json: Value = response.json().unwrap();

    // left side must matched right side
    assert!(json["data"].as_array().unwrap().contains(&families["data"]));

    /*
       Cleanup Section
    */
    delete_test_families(&client, families);
}

#[test]
fn test_create_families() {
    /*
        Setup Section
    */
    let client = setup_client();

    /*
        Test Section
    */
    // response test
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

    // response should be 201
    assert_eq!(response.status(), StatusCode::CREATED);

    // data test
    let families: Value = response.json().unwrap();

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

    /*
        Cleanup Section
    */
    delete_test_families(&client, families);
}

#[test]
fn test_view_families() {
    /*
        Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let uid = families["data"]["id"].as_str().unwrap();

    /*
        Test Section
    */
    // response test
    let response = client
        .get(format!("{}/families/{}", APP_HOST, uid))
        .send()
        .unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let families: Value = response.json().unwrap();

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

    // test view data with dummy id
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .get(format!("{}/families/{}", APP_HOST, dummy_uuid))
        .send()
        .unwrap();

    // response should be 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    /*
        Cleanup Section
    */
    delete_test_families(&client, families);
}

#[test]
fn test_update_families() {
    /*
        Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let uid = families["data"]["id"].as_str().unwrap();

    /*
        Test Section
    */
    // response test
    let response = client
        .put(format!("{}/families/{}", APP_HOST, uid))
        .json(&json!({
            "name": "Modified Family Name",
            "description": "Modified Family Description"
        }))
        .send()
        .unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let families: Value = response.json().unwrap();

    // left side should matched right side
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

    // test update data with dummy id
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .put(format!("{}/families/{}", APP_HOST, dummy_uuid))
        .json(&json!({
            "name": "Modified Family Name",
            "description": "Modified Family Description"
        }))
        .send()
        .unwrap();

    // response should be 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    /*
        Cleanup Section
    */
    delete_test_families(&client, families);
}

#[test]
fn test_delete_families() {
    /*
        Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let uid = families["data"]["id"].as_str().unwrap();

    /*
        Test Section
    */
    // response test
    let response = client
        .delete(format!("{}/families/{}", APP_HOST, uid))
        .send()
        .unwrap();

    // response should be 204
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // test delete data with dummy id
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .delete(format!("{}/families/{}", APP_HOST, dummy_uuid))
        .send()
        .unwrap();

    // response should be 204
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
