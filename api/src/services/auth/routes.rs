use axum::{extract::State, routing::post, Json, Router};
use serde_json::json;
use std::sync::Arc;

/* internal dependency */
use super::{models::UserPasswordAuth, repositories::Repository, types::Routes};
use crate::{
    utils::{errors::ApiError, responses::ApiResponse},
    AppState,
};

impl Routes {
    pub fn init(state: Arc<AppState>) -> Router {
        Router::new()
            .nest(
                "/v1",
                Router::new()
                    .route("/auth/login", post(login))
                    .route("/auth/logout", post(logout)),
            )
            .with_state(state)
    }
}

/*
    [POST] /api/v1/auth/login
    Param:
        * Username => String
        * Password => String
    Return: Result<(), ApiError>
*/
async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UserPasswordAuth>,
) -> Result<ApiResponse, ApiError> {
    let user = Repository::login_by_username(&state.db_pool, &request).await?;
    let role = Repository::get_role_by_username(&state.db_pool, &user).await?;
    let token = Repository::authorized_user(&state.config, &user, &role, &request).await?;

    Ok(ApiResponse {
        data: Some(json!({"token": token})),
        ..ApiResponse::default()
    })
}

/*
    [POST] /api/v1/auth/logout
    Param: User ID => Uuid
    Return: Result<(), ApiError>
*/
async fn logout() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse {
        message: String::from("from logout"),
        ..ApiResponse::default()
    })
}
