pub mod common;
pub mod services;
pub mod utils;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    Router,
};
use core::fmt::Debug;
use std::sync::Arc;
use utils::errors::ApiError;

#[derive(Debug, FromRef, Clone)]
pub struct AppState {
    pub config: common::configs::Config,
    pub db_pool: common::databases::DbPool,
    pub cache_pool: common::caches::CachePool,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync + Debug,
{
    type Rejection = ApiError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

pub async fn register_routes(state: Arc<AppState>) -> Router {
    Router::new().nest_service(
        "/api",
        Router::new()
            .merge(services::families::Routes::init(state.clone()))
            .merge(services::auth::Routes::init(state.clone())),
    )
}
