use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder,
};

use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Display, PartialEq, Serialize)]
pub enum ZenError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeTokenError(String),
    CannotEncodeTokenError(String),
    InternalServerError(String),
    NotFound(String),
    DataBaseError(String),

    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    errors: Vec<T>,
}

impl<String> Deref for ErrorResponse<String> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

/// 自定义错误
impl ResponseError for ZenError {
    fn status_code(&self) -> StatusCode {
        match self {
            ZenError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ZenError::NotFound(_) => StatusCode::NOT_FOUND,
            ZenError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ZenError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
    }
}

//  match self {
//             ServError::BadRequest(error) => StatusCode::BAD_REQUEST,
//             ServError::NotFound(_) => StatusCode::NOT_FOUND,
//             ServError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
//             ServError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         }

/// 将Vec<String>转化为ErrorResponse
impl From<Vec<String>> for ErrorResponse<String> {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ServiceErrors
impl From<DBError> for ZenError {
    fn from(error: DBError) -> ZenError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ZenError::BadRequest(message);
                }
                ZenError::InternalServerError("Unknown database error".into())
            }
            _ => ZenError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for ZenError {
    fn from(error: PoolError) -> ZenError {
        ZenError::DataBaseError(error.to_string())
    }
}

/// Convert BlockingError to ServiceErrors
impl From<BlockingError> for ZenError {
    fn from(error: BlockingError) -> ZenError {
        ZenError::BlockingError(error.to_string())
    }
}
