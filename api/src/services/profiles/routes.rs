use axum::{extract::State, routing::get, Router};
use serde_json::json;
use std::sync::Arc;

/* internal dependencies */
use super::{repositories::Repository, Routes};
use crate::{
    services::auth::Claims,
    utils::{errors::ApiError, responses::ApiResponse},
    AppState,
};

impl Routes {
    pub fn init(state: Arc<AppState>) -> Router {
        Router::new()
            .nest("/v1", Router::new().route("/profiles", get(get_profiles)))
            .with_state(state)
    }
}

/*
    [GET] /api/v1/profiles
    Param: None
    Return: Result<ApiResponse, ApiError>
*/
async fn get_profiles(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse, ApiError> {
    let profile = Repository::get_profile_by_id(&state.db_pool, claims.sub).await?;

    Ok(ApiResponse {
        data: Some(json!(profile)),
        message: String::from("from get profile"),
        ..ApiResponse::default()
    })
}
