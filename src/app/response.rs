use actix_web::{
    http::{
        header::{Accept, ContentType},
        StatusCode,
    },
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
    Response helper
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub response_code: u16,
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<Value>,
    pub errors: Option<Value>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names)]
pub enum ResponseVariant {
    OkResponse,
    FailedResponse(FailedResponseCode),
    ErrorResponse(ErrorResponseCode),
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

impl Response {
    pub fn new(
        response_code: u16,
        code: u16,
        message: String,
        data: Option<Value>,
        errors: Option<Value>,
    ) -> Self {
        let response_variant = match ResponseVariant::try_from(response_code) {
            Ok(r) => r,
            Err(_) => ResponseVariant::ErrorResponse(ErrorResponseCode::UnknownError),
        };

        Self {
            response_code,
            status: response_variant.to_string(),
            code,
            message,
            data,
            errors,
        }
    }

    pub fn return_ok(&self) -> HttpResponse {
        let response_data = self.build_response();

        HttpResponse::build(StatusCode::from_u16(self.response_code).unwrap())
            .insert_header(ContentType::json())
            .insert_header(Accept::json())
            .json(&response_data)
    }

    pub fn return_failed(&self) -> HttpResponse {
        let response_data = self.build_response();

        HttpResponse::build(StatusCode::from_u16(self.response_code).unwrap())
            .insert_header(ContentType::json())
            .insert_header(Accept::json())
            .json(&response_data)
    }

    pub fn return_error(&self) -> HttpResponse {
        let response_data = self.build_response();

        HttpResponse::build(StatusCode::from_u16(self.response_code).unwrap())
            .insert_header(ContentType::json())
            .insert_header(Accept::json())
            .json(&response_data)
    }

    fn build_response(&self) -> Value {
        json!({
            "status": self.status,
            "code": self.code,
            "message": self.message,
            "data": match &self.data {
                Some(d) => d.clone(),
                None => json!({}),
            },
            "errors": match &self.errors {
                Some(d) => d.clone(),
                None => json!([])
            },
        })
    }
}

impl ToString for ResponseVariant {
    fn to_string(&self) -> String {
        match self {
            ResponseVariant::OkResponse => String::from("success"),
            ResponseVariant::FailedResponse(e) => match e {
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
            ResponseVariant::ErrorResponse(e) => match e {
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

impl TryFrom<u16> for ResponseVariant {
    type Error = ();

    fn try_from(response_code: u16) -> Result<Self, Self::Error> {
        match response_code {
            200 => Ok(ResponseVariant::OkResponse),

            /* 4xx error code */
            400 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::BadRequest,
            )),
            401 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::Unauthorized,
            )),
            403 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::Forbidden,
            )),
            404 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::NotFound,
            )),
            405 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::MethodNotAllowed,
            )),
            406 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::NotAcceptable,
            )),
            408 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::RequestTimeout,
            )),
            409 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::Conflict,
            )),
            415 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::UnsupportedMediaType,
            )),
            422 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::UnprocessableContent,
            )),
            429 => Ok(ResponseVariant::FailedResponse(
                FailedResponseCode::TooManyRequest,
            )),

            /* 5xx error code */
            500 => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::InternalServerError,
            )),
            501 => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::NotImplemented,
            )),
            502 => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::BadGateway,
            )),
            503 => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::ServiceUnavailable,
            )),
            504 => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::GatewayTimeout,
            )),

            _ => Ok(ResponseVariant::ErrorResponse(
                ErrorResponseCode::UnknownError,
            )),
        }
    }
}
