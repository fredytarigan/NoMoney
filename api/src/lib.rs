pub mod common;
pub mod services;
pub mod utils;

use axum::Router;
use std::sync::Arc;

pub struct AppState {
    pub config: common::configs::Config,
    pub db_pool: common::databases::DbPool,
    pub cache_pool: common::caches::CachePool,
}

pub async fn register_routes(state: Arc<AppState>) -> Router {
    Router::new().nest_service(
        "/api",
        Router::new()
            .merge(services::families::Routes::init(state.clone()))
            .merge(services::auth::Routes::init(state.clone())),
    )
}
