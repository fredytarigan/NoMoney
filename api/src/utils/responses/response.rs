use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

/* internal dependency */
use crate::utils::responses::AppResponse;

impl Default for AppResponse {
    fn default() -> Self {
        Self {
            code: 200,
            status: String::from("success"),
            message: String::new(),
            data: Some(json!("[]")),
            errors: Some(json!("[]")),
        }
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.code).expect("invalid response code");
        let body = json!({
                "code": self.code,
                "message": self.message,
                "data": self.data,
                "errors": self.errors
        });

        (status_code, Json(body)).into_response()
    }
}
