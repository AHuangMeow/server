use crate::constants::{INTERNAL_SERVER_ERROR, INVALID_USER_ID};
use crate::models::response::Response;

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

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
        match self {
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(Response::<()> {
                msg: msg.into(),
                data: None,
            }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(Response::<()> {
                msg: msg.into(),
                data: None,
            }),
            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(Response::<()> {
                msg: msg.into(),
                data: None,
            }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(Response::<()> {
                msg: msg.into(),
                data: None,
            }),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(Response::<()> {
                msg: msg.into(),
                data: None,
            }),
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                HttpResponse::InternalServerError().json(Response::<()> {
                    msg: INTERNAL_SERVER_ERROR.into(),
                    data: None,
                })
            }
            AppError::Redis(e) => {
                tracing::error!("Redis error: {:?}", e);
                HttpResponse::InternalServerError().json(Response::<()> {
                    msg: INTERNAL_SERVER_ERROR.into(),
                    data: None,
                })
            }
            AppError::Internal => {
                tracing::error!("Internal server error");
                HttpResponse::InternalServerError().json(Response::<()> {
                    msg: INTERNAL_SERVER_ERROR.into(),
                    data: None,
                })
            }
        }
    }
}
