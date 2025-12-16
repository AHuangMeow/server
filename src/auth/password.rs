use crate::constants::INVALID_CREDENTIALS;
use crate::errors::AppError;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(plain: &str) -> Result<String, AppError> {
    hash(plain, DEFAULT_COST).map_err(|_| AppError::Internal)
}

pub fn verify_password(hash: &str, plain: &str) -> Result<(), AppError> {
    verify(plain, hash)
        .map_err(|_| AppError::Unauthorized(INVALID_CREDENTIALS.into()))
        .and_then(|ok| {
            if ok {
                Ok(())
            } else {
                Err(AppError::Unauthorized(INVALID_CREDENTIALS.into()))
            }
        })
}
