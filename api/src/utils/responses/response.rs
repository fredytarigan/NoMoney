use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

/* internal dependency */
use crate::utils::responses::ApiResponse;

impl Default for ApiResponse {
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

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());

        let body = json!({
                "code": self.code,
                "message": self.message,
                "data": self.data,
                "errors": self.errors,
                "status": self.status,
        });

        (status_code, headers, Json(body)).into_response()
    }
}
