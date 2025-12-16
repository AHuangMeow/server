use crate::constants::INVALID_USER_ID;
use crate::models::dto::ResultResponse;

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("UnprocessableEntity: {0}")]
    UnprocessableEntity(String),
    #[error("Internal")]
    Internal,
}

impl From<mongodb::error::Error> for AppError {
    fn from(_: mongodb::error::Error) -> Self {
        AppError::Internal
    }
}

impl From<mongodb::bson::oid::Error> for AppError {
    fn from(_: mongodb::bson::oid::Error) -> Self {
        AppError::Unauthorized(INVALID_USER_ID.into())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ResultResponse {
                code: 401,
                msg: msg.into(),
            }),
            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(ResultResponse {
                code: 403,
                msg: msg.into(),
            }),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(ResultResponse {
                code: 409,
                msg: msg.into(),
            }),
            AppError::UnprocessableEntity(msg) => {
                HttpResponse::UnprocessableEntity().json(ResultResponse {
                    code: 422,
                    msg: msg.into(),
                })
            }
            AppError::Internal => HttpResponse::InternalServerError().json(ResultResponse {
                code: 500,
                msg: "internal server error".into(),
            }),
        }
    }
}
