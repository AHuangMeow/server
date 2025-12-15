use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("NotFound")]
    NotFound,
    #[error("Internal")]
    Internal,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::Internal => HttpResponse::InternalServerError().finish(),
        }
    }
}
