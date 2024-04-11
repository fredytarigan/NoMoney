pub mod setup;

use reqwest::StatusCode;
use serde_json::Value;
use setup::{setup_client, APP_HOST};

#[test]
fn test_get_roles() {
    /*
       Setup Section
    */
    let client = setup_client();

    /*
       Test Section
    */
    // response test
    let response = client.get(format!("{}/roles", APP_HOST)).send().unwrap();

    // response should be 200
    assert_eq!(response.status(), StatusCode::OK);

    // data test
    let roles: Value = response.json().unwrap();

    // total row must be 3
    assert_eq!(roles["data"].as_array().unwrap().len(), 3);
}
