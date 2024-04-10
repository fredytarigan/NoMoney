use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ApplicationError {
    pub message: String,
    pub status: u16,
}

impl ApplicationError {
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_json = serde_json::to_string(self).unwrap();

        write!(f, "{}", err_json)
    }
}

impl From<DieselError> for ApplicationError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => {
                ApplicationError::new(500, err.message().to_string())
            }
            DieselError::NotFound => ApplicationError::new(404, String::from("Record not found")),

            _ => ApplicationError::new(500, format!("Unhandled database error: {}", error)),
        }
    }
}

impl From<RunError> for ApplicationError {
    fn from(error: RunError) -> Self {
        match error {
            RunError::TimedOut => ApplicationError::new(
                500,
                String::from("Server error: connection timed out to database"),
            ),
            _ => ApplicationError::new(500, format!("Unhandled database error: {}", error)),
        }
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let err_json = serde_json::json!({
            "status": "error",
            "data": {},
            "message": self.message
        });

        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .insert_header(ContentType::json())
            .json(err_json)
    }
}
