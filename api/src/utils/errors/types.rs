use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ApiError {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
    pub errors: Option<Value>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names)]
pub enum ErrorVariant {
    ResponseFailed(FailedResponseCode),
    ResponseError(ErrorResponseCode),
}

/*
    FailedResponseCode is collection of 4xx response code
    * BadRequest            400
    * Unauthorizde          401
    * Forbidden             403
    * NotFound              404
    * MethodNotAllowed      405
    * NotAcceptable         406
    * RequestTimeout        408
    * Conflict               409
    * UnsupportedMediaType  415
    * UnprocessableContent  422
    * TooManyRequest        429
*/
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum FailedResponseCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    Conflict,
    UnsupportedMediaType,
    UnprocessableContent,
    TooManyRequest,
}

/*
    ErrorResponseCode is collection fo 5xx response code
    * InternalServerError   500
    * NotImplemented        501
    * BadGateway            502
    * ServiceUnavailable    503
    * GatewayTimeout        504
    * UnknwonError          *
*/
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorResponseCode {
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    UnknownError,
}
