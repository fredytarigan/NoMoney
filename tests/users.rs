pub mod setup;

use reqwest::StatusCode;
use serde_json::{json, Value};
use setup::{families::create_test_families, setup_client, APP_HOST};

use crate::setup::{
    families::delete_test_families,
    users::{create_test_users, delete_test_users},
};

#[test]
fn test_get_users() {
    /*
       Setup Section
    */
    let client = setup_client();
    let families: Value = create_test_families(&client);

    // get families id
    let family_id = families["data"]
        .as_object()
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();

    let role = String::from("viewer");
    let username = String::from("test.user.viewer.get");
    let users = create_test_users(&client, &family_id.to_string(), role, &username);

    /*
       Test Section
    */
    // response test
    let response = client.get(format!("{}/users", APP_HOST)).send().unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let json: Value = response.json().unwrap();

    // left side must matched right side
    assert!(json["data"].as_array().unwrap().contains(&users["data"]));

    /*
       Cleanup Section
    */
    delete_test_users(&client, users);
    delete_test_families(&client, families);
}

#[test]
fn test_create_users() {
    /*
       Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let family_id = families["data"]["id"].as_str().unwrap();
    let roles = client
        .get(format!("{}/roles/search?name=viewer", APP_HOST))
        .send()
        .unwrap();
    let roles_result: Value = roles.json().unwrap();
    let role_id = roles_result["data"]["id"].as_str().unwrap();

    /*
       Test Section
    */
    // response test
    let username = String::from("test.user.viewer.create");
    let response = client
        .post(format!("{}/users", APP_HOST))
        .json(&json!(
            {
                "username": username,
                "password": "12345678",
                "active": true,
                "family_id": family_id,
                "role_id": role_id,
                "email": "test.user.viewer@example.org",
                "email_validated": true,
                "first_name": "Test",
                "last_name": "Viewer"
            }
        ))
        .send()
        .unwrap();

    // response should be 201
    assert_eq!(response.status(), StatusCode::CREATED);

    // data test
    let users: Value = response.json().unwrap();

    // left side must matched right side
    assert_eq!(
        users["data"],
        json!(
            {
                "id": users["data"]["id"],
                "username": username,
                "active": true,
                "family_id": family_id,
                "role_id": role_id,
                "email": "test.user.viewer@example.org",
                "email_validated": true,
                "first_name": "Test",
                "last_name": "Viewer",
                "created_at": users["data"]["created_at"],
                "updated_at": users["data"]["updated_at"],
            }
        )
    );

    /*
       Cleanup Section
    */
    delete_test_users(&client, users);
    delete_test_families(&client, families);
}

#[test]
fn test_view_users() {
    /*
       Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let family_id = families["data"]["id"].as_str().unwrap();
    let role = String::from("viewer");
    let username = String::from("test.user.viewer.view");
    let users = create_test_users(&client, &family_id.to_string(), role, &username);
    let uid = users["data"]["id"].as_str().unwrap();

    /*
       Test Section
    */
    // response test
    let response = client
        .get(format!("{}/users/{}", APP_HOST, uid))
        .send()
        .unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let users: Value = response.json().unwrap();

    // left side must matched right side
    assert_eq!(
        users["data"],
        json!(
            {
                "id": users["data"]["id"],
                "username": username,
                "active": true,
                "family_id": users["data"]["family_id"],
                "role_id": users["data"]["role_id"],
                "email": "test.user.viewer@example.org",
                "email_validated": true,
                "first_name": "Test",
                "last_name": "viewer",
                "created_at": users["data"]["created_at"],
                "updated_at": users["data"]["updated_at"],
            }
        )
    );

    /*
       Cleanup Section
    */
    delete_test_users(&client, users);
    delete_test_families(&client, families);
}

#[test]
fn test_delete_users() {
    /*
       Setup Section
    */
    let client = setup_client();
    let families = create_test_families(&client);
    let family_id = families["data"]["id"].as_str().unwrap();
    let role = String::from("viewer");
    let username = String::from("test.user.viewer.delete");
    let users = create_test_users(&client, &family_id.to_string(), role, &username);
    let uid = users["data"]["id"].as_str().unwrap();

    /*
       Test Section
    */
    // response test
    let response = client
        .delete(format!("{}/users/{}", APP_HOST, uid))
        .send()
        .unwrap();

    // response should be 204
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // test delete data with dummy id
    let dummy_uuid = uuid::Uuid::new_v4();
    let response = client
        .delete(format!("{}/users/{}", APP_HOST, dummy_uuid))
        .send()
        .unwrap();

    // response should be 204
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
