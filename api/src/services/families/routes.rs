use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{response::IntoResponse, Router};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

use super::mocks;
/* internal dependency */
use super::types::Routes;
use crate::utils::responses::AppResponse;
use crate::AppState;

impl Routes {
    pub fn init(state: Arc<AppState>) -> Router {
        Router::new()
            .nest(
                "/v1",
                Router::new()
                    .route("/families", get(list_families))
                    .route("/families/:id", get(get_families))
                    .route("/families", post(create_families)),
            )
            .with_state(state)
    }
}

/*
    [GET] /api/v1/families
    Param: None
    Return: Result<(), Error>
*/
async fn list_families() -> impl IntoResponse {
    let result = mocks::list_families().await;

    AppResponse {
        data: Some(json!(result.unwrap())),
        message: String::from("from list families"),
        ..AppResponse::default()
    }
}

/*
    [GET] /api/v1/families/:id
    Param:
        * Family ID => UUID
    Return: Result<(), Error>
*/
async fn get_families() -> impl IntoResponse {
    "I am get families"
}

/*
    [POST] /api/v1/families
    Param:
        * Family Data => CreateFamily
    Return: Result<(), Error>
*/
async fn create_families() -> impl IntoResponse {
    "I am create families"
}
