use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ApplicationError {
    pub message: String,
    pub status: u16,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_json = serde_json::to_string(self).unwrap();

        write!(f, "{}", err_json)
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let err_json = serde_json::json!({
            "result": "error",
            "message": self.message
        });

        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .insert_header(ContentType::json())
            .json(err_json)
    }
}
