use crate::constants::{INTERNAL_SERVER_ERROR, INVALID_USER_ID};
use crate::models::response::Response;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Internal server error")]
    Internal,
}

impl From<mongodb::bson::oid::Error> for AppError {
    fn from(_: mongodb::bson::oid::Error) -> Self {
        AppError::Unauthorized(INVALID_USER_ID.into())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        fn json_error(status: StatusCode, msg: String) -> HttpResponse {
            HttpResponse::build(status).json(Response::<()> { msg, data: None })
        }

        match self {
            AppError::BadRequest(msg) => json_error(StatusCode::BAD_REQUEST, msg.into()),
            AppError::Unauthorized(msg) => json_error(StatusCode::UNAUTHORIZED, msg.into()),
            AppError::Forbidden(msg) => json_error(StatusCode::FORBIDDEN, msg.into()),
            AppError::NotFound(msg) => json_error(StatusCode::NOT_FOUND, msg.into()),
            AppError::Conflict(msg) => json_error(StatusCode::CONFLICT, msg.into()),
            AppError::Database(e) => {
                error!("Database error: {:?}", e);
                json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    INTERNAL_SERVER_ERROR.into(),
                )
            }
            AppError::Redis(e) => {
                error!("Redis error: {:?}", e);
                json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    INTERNAL_SERVER_ERROR.into(),
                )
            }
            AppError::Internal => {
                error!("Internal server error");
                json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    INTERNAL_SERVER_ERROR.into(),
                )
            }
        }
    }
}
