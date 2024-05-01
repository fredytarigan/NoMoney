pub mod common;
pub mod services;
pub mod utils;

use axum::Router;
use std::sync::Arc;

pub struct AppState {
    pub config: common::configs::Config,
}

pub async fn register_routes(state: Arc<AppState>) -> Router {
    Router::new().nest_service("/api", services::families::Routes::init(state.clone()))
}
