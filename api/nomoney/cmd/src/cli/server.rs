use std::time::Duration;

use anyhow::Result;
use axum::{extract::Request, Router, ServiceExt};
use tower_http::{normalize_path::NormalizePathLayer, timeout::TimeoutLayer};
use tower_layer::Layer;
use tracing::info;

pub async fn run() -> Result<()> {
    // load application config
    let config = app_commons::Config::init();

    // initialize application logger
    let logger = app_commons::Logger::new(&config);
    let _ = logger.init();

    // setup axum app
    let app = NormalizePathLayer::trim_trailing_slash().layer(Router::new().layer(
        TimeoutLayer::new(Duration::from_secs(config.server_timeout)),
    ));

    // set listener addr and port from configuration
    let listener =
        tokio::net::TcpListener::bind(&format!("{}:{}", config.server_addr, config.server_port))
            .await?;

    /*
      show some information about server
      so we now that we alive
    */
    info!(
        "Setup server with listener at {}:{}",
        &config.server_addr, &config.server_port
    );

    // start axum server
    let _ = axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
