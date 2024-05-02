use std::sync::Arc;

use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get};
use axum::{Json, Router};
use nomoney::services::users::Person;
use nomoney::AppState;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    /*
        Load application configuration
    */
    let config = nomoney::common::configs::Config::default();

    /*
       Initialize application logger
    */
    let _ = nomoney::common::loggers::Logger::new(&config).init();
    info!(
        "successfully configure logging with level {:?}",
        &config.base.env
    );

    /*
       Initialize database connection pool
    */
    let db = nomoney::common::databases::Database::new(&config).await;
    let db_pool = db.init().await;

    /* run pending migration if exists */
    let _ = nomoney::common::databases::Database::run_migration(db_pool.clone()).await;

    /*
       Initialize caching connection pool
    */
    let cache = nomoney::common::caches::Cache::new(&config).await;
    let cache_pool = cache.init().await;

    let cors = CorsLayer::new().allow_origin(Any);

    /*
       Set application state object
    */
    let state = Arc::new(AppState {
        config: config.clone(),
        db_pool: db_pool.clone(),
        cache_pool: cache_pool.clone(),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/healthz", get(healthz))
        .route("/people", get(get_people))
        /*
            Register API Handler
        */
        .merge(nomoney::register_routes(state).await)
        /*
            Register UI from react app
        */
        .nest_service("/ui", ServeDir::new("dist"))
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind(&format!("{}:{}", &config.server.addr, &config.server.port))
            .await
            .unwrap();

    info!(
        "server start listening at {} on port {}",
        &config.server.addr, &config.server.port
    );

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello World !!!"
}

async fn healthz() -> &'static str {
    "Ok Healthy"
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}
