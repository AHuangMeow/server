use crate::config::AppConfig;
use crate::constants::AUTH_REQUIRED;
use crate::errors::AppError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

pub fn generate_token(cfg: &AppConfig, user_id: &str) -> Result<String, AppError> {
    let exp =
        (OffsetDateTime::now_utc() + Duration::hours(cfg.jwt_exp_hours)).unix_timestamp() as usize;
    let claims = Claims {
        sub: user_id.into(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)
}

pub fn decode_token(cfg: &AppConfig, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized(AUTH_REQUIRED.into()))
}
