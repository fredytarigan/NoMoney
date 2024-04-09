use actix_web::{get, web::Json, App, HttpServer, Responder, Result};
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

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            // default route (/)
            .service(index)
            // health check route (/healthz)
            .service(healthz)
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();

    server.await
}
