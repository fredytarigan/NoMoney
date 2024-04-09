use actix_web::{get, middleware::Logger, web::Json, App, HttpServer, Responder, Result};
use serde_json::json;

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

    let server = HttpServer::new(|| {
        App::new()
            // default route (/)
            .service(index)
            // health check route (/healthz)
            .service(healthz)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();

    server.await
}
