use crate::models::dto::ResultResponse;

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("UnprocessableEntity: {0}")]
    UnprocessableEntity(String),
    #[error("Internal")]
    Internal,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ResultResponse {
                code: 400,
                msg: msg.into(),
            }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ResultResponse {
                code: 401,
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
