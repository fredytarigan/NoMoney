/* */
#[macro_use]
extern crate log;

use actix_web::{
    get, guard,
    middleware::{self, Logger},
    web::{self, Json},
    App, HttpServer, Responder, Result,
};
use database::run_db_migrations;
use env_logger::Env;
use serde_json::json;

mod app;
mod database;
mod errors;
mod redis;
mod schema;

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(Json(json!({
        "status": "success",
        "data": {},
        "message": "Hello World From NoMoney",
    })))
}

#[get("/healthz")]
async fn healthz() -> Result<impl Responder> {
    Ok(Json(json!({
        "status": "success",
        "data": {},
        "message": "application is running and healthy"
    })))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // load configuration from app.env file
    dotenvy::from_filename("./config/app.env").ok();

    // setup logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // prepare server configuration
    let server_host = dotenvy::var("SERVER_HOST").unwrap_or(String::from("127.0.0.1"));
    let server_port = dotenvy::var("SERVER_PORT").unwrap_or(String::from("8080"));
    let server_address = format!("{}:{}", server_host, server_port);

    // load database connection
    let database_connection = database::initialize_db_pool().await;

    // load redis connection
    let redis_connection = redis::initialize_redis_pool().await;

    // run db migrations
    run_db_migrations().await;

    info!("Running server");
    info!("Listening on {} at port {}", server_host, server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("content-type", "application/json")))
            .wrap(middleware::DefaultHeaders::new().add(("X-Server", "NoMoney")))
            .app_data(web::Data::new(database_connection.clone()))
            .app_data(web::Data::new(redis_connection.clone()))
            // default root route ( / )
            .service(index)
            // health check route ( /healthz )
            .service(healthz)
            // api service
            .service(
                // v1 api scope
                web::scope("/api/v1")
                    .guard(guard::Header("content-type", "application/json"))
                    // route for family
                    .configure(app::register_routes),
            )
    })
    .bind(&server_address)?
    .run()
    .await
}
