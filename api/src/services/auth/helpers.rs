use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::extract::FromRef;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use rand::rngs::OsRng;
use serde_json::json;
use std::fmt::Debug;
use std::sync::Arc;

use super::repositories::Repository;
/* internal dependencies */
use super::types::{AuthError, Claims, Keys};
use crate::{
    utils::errors::{ApiError, ErrorResponseCode, ErrorVariant},
    AppState,
};

pub fn hash_password(password: String) -> Result<String, ApiError> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();

    let hashed_password = argon.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
        };

        let body = json!(
            {
                "code": status.as_u16(),
                "message": error_message,
                "data": Some(json!("{}")),
                "status": "error",
                "errors": Some(json!("[auth error]")),
            }
        );

        (status, Json(body)).into_response()
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        let (status, error_message) = match error {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
        };

        Self {
            code: status.as_u16(),
            message: String::from(error_message),
            data: Some(json!("{}")),
            status: ErrorVariant::try_from(status.as_u16())
                .unwrap_or(ErrorVariant::ResponseError(
                    ErrorResponseCode::InternalServerError,
                ))
                .to_string(),
            errors: Some(json!("[auth error]")),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync + Debug,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let state = Arc::from_ref(state);
        let state = parts
            .extract_with_state::<AppState, _>(&state)
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        let keys = Keys::new(state.config.jwt.secret.as_bytes());

        let token_data = decode::<Claims>(bearer.token(), &keys.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        /* check if user is active */
        match Repository::check_user_is_active(
            &state.db_pool,
            &token_data.claims.sub,
            &token_data.claims.username,
        )
        .await
        {
            true => return Ok(token_data.claims),
            false => return Err(AuthError::WrongCredentials),
        };
    }
}
