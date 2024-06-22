use std::time::Duration;

use anyhow::Result;
use axum::{extract::Request, Router, ServiceExt};
use tower_http::{normalize_path::NormalizePathLayer, timeout::TimeoutLayer};
use tower_layer::Layer;

pub async fn run() -> Result<()> {
    // load application config
    let config = app_commons::Config::init();

    // setup axum app
    let app = NormalizePathLayer::trim_trailing_slash().layer(Router::new().layer(
        TimeoutLayer::new(Duration::from_secs(config.server_timeout)),
    ));

    // set listener addr and port from configuration
    let listener =
        tokio::net::TcpListener::bind(&format!("{}:{}", config.server_addr, config.server_port))
            .await?;

    // start axum server
    let _ = axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
