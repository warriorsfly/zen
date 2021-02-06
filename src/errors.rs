use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde::{Deserialize, Serialize};
use uuid::Error as UuidError;

#[derive(Debug, Display, PartialEq)]
pub enum ServError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    DecodeTokenError(String),
    EncodeTokenError(String),
    InternalServerError(String),
    NotFound(String),
    UuidError(String),
    DataBaseError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// 自定义错误
impl ResponseError for ServError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ServError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ServError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            }
            ServError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
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

/// Convert DBErrors to ServiceErrors
impl From<DBError> for ServError {
    fn from(error: DBError) -> ServError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServError::BadRequest(message);
                }
                ServError::InternalServerError("Unknown database error".into())
            }
            _ => ServError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for ServError {
    fn from(error: PoolError) -> ServError {
        ServError::DataBaseError(error.to_string())
    }
}

/// Convert ParseErrors to ServiceErrors
impl From<UuidError> for ServError {
    fn from(error: UuidError) -> ServError {
        ServError::UuidError(error.to_string())
    }
}

/// Convert Thread BlockingErrors to ServiceErrors
impl From<BlockingError<ServError>> for ServError {
    fn from(error: BlockingError<ServError>) -> ServError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ServError::BlockingError("Thread blocking error".into()),
        }
    }
}
