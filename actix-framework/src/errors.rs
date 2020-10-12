use actix_web::{error::ResponseError, HttpResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Bad request:{0}")]
    BadRequest(String),
    #[error("Internal server error:{0}")]
    InternalServerError(String),
    #[error("Not found:{0}")]
    NotFound(String),
    #[error("Unauthorized:{0}")]
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// 自定义错误
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ServiceError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ServiceError::InternalServerError(message) => {
                HttpResponse::InternalServerError().json::<ErrorResponse>(message.into())
            }
            ServiceError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
        }
    }
}

/// 将String转化为ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// 将Vec<String>转化为ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}
