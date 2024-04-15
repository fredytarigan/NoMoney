use actix_web::ResponseError;
use argon2::password_hash::Error as HashError;
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError;
use serde::Serialize;
use serde_json::json;
use std::fmt::Display;

use crate::app::Response;

#[derive(Serialize, Debug)]
pub struct ApplicationError {
    response: Response,
}

impl ApplicationError {
    pub fn new(response: Response) -> Self {
        Self { response }
    }

    #[allow(dead_code)]
    pub fn return_error(&self) {
        self.response.return_error();
    }

    #[allow(dead_code)]
    pub fn return_failed(&self) {
        self.response.return_failed();
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
            DieselError::DatabaseError(_, _) => {
                let response = Response::new(
                    500,
                    5000,
                    String::from("database connection error"),
                    None,
                    Some(json!(["database error"])),
                );

                ApplicationError::new(response)
            }
            DieselError::NotFound => {
                let response = Response::new(
                    200,
                    4004,
                    String::from("requested resources not found"),
                    None,
                    None,
                );

                ApplicationError::new(response)
            }

            _ => {
                let response = Response::new(
                    500,
                    5000,
                    String::from("unhandled error happen at server side"),
                    None,
                    Some(json!(["database error"])),
                );

                ApplicationError::new(response)
            }
        }
    }
}

impl From<RunError> for ApplicationError {
    fn from(error: RunError) -> Self {
        match error {
            RunError::TimedOut => {
                let response = Response::new(
                    500,
                    5000,
                    String::from("database connection timeout"),
                    None,
                    Some(json!(["database error"])),
                );

                ApplicationError::new(response)
            }
            _ => {
                let response = Response::new(
                    500,
                    5000,
                    String::from("unhandled error happen at server side"),
                    None,
                    Some(json!(["database error"])),
                );

                ApplicationError::new(response)
            }
        }
    }
}

impl From<HashError> for ApplicationError {
    fn from(_: HashError) -> Self {
        {
            error!("Something error when trying to hash incoming password");

            let response = Response::new(
                500,
                5000,
                String::from("crypto hashing error"),
                None,
                Some(json!(["hashing error"])),
            );

            ApplicationError::new(response)
        }
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self.response.code {
            400..=499 => self.response.return_failed(),
            500..=599 => self.response.return_error(),

            _ => self.response.return_error(),
        }
    }
}
