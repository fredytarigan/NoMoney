use api::services::users::Person;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get};
use axum::{Json, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .nest_service("/ui", ServeDir::new("dist"))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:8083"))
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello World !!!"
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
