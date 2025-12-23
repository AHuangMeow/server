use crate::config::app_config::AppConfig;
use crate::constants::AUTH_REQUIRED;
use crate::errors::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

fn default_claims_ver() -> i32 {
    0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
    pub iat: usize, // issued at
    #[serde(default = "default_claims_ver")]
    pub ver: i32,
}

pub fn generate_token(cfg: &AppConfig, user_id: &str, token_version: i32) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp() as usize;
    let exp =
        (OffsetDateTime::now_utc() + Duration::hours(cfg.jwt_exp_hours)).unix_timestamp() as usize;
    let claims = Claims {
        sub: user_id.into(),
        exp,
        iat: now,
        ver: token_version,
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
    .map_err(|e| {
        tracing::warn!("Token decode error: {:?}", e);
        AppError::Unauthorized(AUTH_REQUIRED.into())
    })
}
