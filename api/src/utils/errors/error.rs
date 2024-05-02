use argon2::password_hash::Error as HashError;
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::error::Error as SqlxError;

/* internal dependency */
use super::{ApiError, ErrorResponseCode, ErrorVariant, FailedResponseCode};

impl Default for ApiError {
    fn default() -> Self {
        Self {
            code: 500,
            status: ErrorVariant::ResponseError(ErrorResponseCode::InternalServerError).to_string(),
            message: String::new(),
            data: Some(json!("[]")),
            errors: Some(json!("[unhandled error]")),
        }
    }
}

impl IntoResponse for ApiError {
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
            "status": self.status,
            "errors": self.errors,
        });

        (status_code, headers, Json(body)).into_response()
    }
}

impl From<SqlxError> for ApiError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::Database(_) => ApiError {
                message: String::from("database connection errors"),
                errors: Some(json!("[database error]")),
                ..ApiError::default()
            },
            SqlxError::RowNotFound => ApiError {
                message: String::from("resources not found"),
                errors: Some(json!("[]")),
                code: 200,
                status: String::from("success"),
                ..ApiError::default()
            },

            _ => ApiError {
                message: String::from("unhandled database error"),
                errors: Some(json!("[database error]")),
                ..ApiError::default()
            },
        }
    }
}

impl From<HashError> for ApiError {
    fn from(_: HashError) -> Self {
        ApiError {
            message: String::from("crypto hashing error"),
            errors: Some(json!("[hashing error]")),
            ..ApiError::default()
        }
    }
}

impl ToString for ErrorVariant {
    fn to_string(&self) -> String {
        match self {
            ErrorVariant::ResponseFailed(e) => match e {
                FailedResponseCode::BadRequest => String::from("bad request"),
                FailedResponseCode::Unauthorized => String::from("unauthorized"),
                FailedResponseCode::Forbidden => String::from("forbidden"),
                FailedResponseCode::NotFound => String::from("not found"),
                FailedResponseCode::MethodNotAllowed => String::from("method not allowed"),
                FailedResponseCode::NotAcceptable => String::from("not acceptable"),
                FailedResponseCode::RequestTimeout => String::from("request timeout"),
                FailedResponseCode::Conflict => String::from("conflict"),
                FailedResponseCode::UnsupportedMediaType => String::from("unsupported media type"),
                FailedResponseCode::UnprocessableContent => String::from("unprocessable content"),
                FailedResponseCode::TooManyRequest => String::from("too many request"),
            },
            ErrorVariant::ResponseError(e) => match e {
                ErrorResponseCode::InternalServerError => String::from("internal server error"),
                ErrorResponseCode::NotImplemented => String::from("not implemented"),
                ErrorResponseCode::BadGateway => String::from("bad gateway"),
                ErrorResponseCode::ServiceUnavailable => String::from("service unavailable"),
                ErrorResponseCode::GatewayTimeout => String::from("gateway timeout"),
                ErrorResponseCode::UnknownError => String::from("unknown error"),
            },
        }
    }
}

impl TryFrom<u16> for ErrorVariant {
    type Error = ();

    fn try_from(response_code: u16) -> Result<Self, Self::Error> {
        match response_code {
            /* 4xx error code */
            400 => Ok(ErrorVariant::ResponseFailed(FailedResponseCode::BadRequest)),
            401 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::Unauthorized,
            )),
            403 => Ok(ErrorVariant::ResponseFailed(FailedResponseCode::Forbidden)),
            404 => Ok(ErrorVariant::ResponseFailed(FailedResponseCode::NotFound)),
            405 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::MethodNotAllowed,
            )),
            406 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::NotAcceptable,
            )),
            408 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::RequestTimeout,
            )),
            409 => Ok(ErrorVariant::ResponseFailed(FailedResponseCode::Conflict)),
            415 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::UnsupportedMediaType,
            )),
            422 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::UnprocessableContent,
            )),
            429 => Ok(ErrorVariant::ResponseFailed(
                FailedResponseCode::TooManyRequest,
            )),

            /* 5xx error code */
            500 => Ok(ErrorVariant::ResponseError(
                ErrorResponseCode::InternalServerError,
            )),
            501 => Ok(ErrorVariant::ResponseError(
                ErrorResponseCode::NotImplemented,
            )),
            502 => Ok(ErrorVariant::ResponseError(ErrorResponseCode::BadGateway)),
            503 => Ok(ErrorVariant::ResponseError(
                ErrorResponseCode::ServiceUnavailable,
            )),
            504 => Ok(ErrorVariant::ResponseError(
                ErrorResponseCode::GatewayTimeout,
            )),

            _ => Ok(ErrorVariant::ResponseError(ErrorResponseCode::UnknownError)),
        }
    }
}
