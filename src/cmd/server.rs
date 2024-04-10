use actix_web::{
    get,
    middleware::Logger,
    web::{self, Json},
    App, HttpServer, Responder, Result,
};
use serde_json::json;

use crate::routes::{self, family::RouteFamily};

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(Json(json!({
        "result": "ok",
        "message": "Hello World",
    })))
}

#[get("/healthz")]
async fn healthz() -> Result<impl Responder> {
    Ok(Json(json!({
        "result": "ok",
        "message": "healthy"
    })))
}

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    // initialize database pool
    let pool = routes::initialize_db_pool().await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            // default route (/)
            .service(index)
            // health check route (/healthz)
            .service(healthz)
            // api service
            .service(
                // api v1 scope
                web::scope("/api/v1")
                    // route for family
                    .configure(RouteFamily::route),
            )
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();

    server.await
}
