use reqwest::{blocking::Client, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use super::APP_HOST;

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub code: u16,
    pub data: Value,
    pub errors: Option<Vec<String>>,
    pub message: String,
    pub status: String,
}

pub fn login_as_default_admin(client: &Client) -> String {
    let response = client
        .post(format!("{}/auth/login", APP_HOST))
        .json(&json!(
            {
                "username": "admin",
                "password": "admin",
            }
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let result = response.json::<LoginResponse>().unwrap();

    assert_eq!(
        result.data,
        json!({
            "token": result.data["token"]
        })
    );

    return result.data["token"].to_string().replace("\"", "");
}
