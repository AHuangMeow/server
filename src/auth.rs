use crate::{config::AppConfig, errors::AppError, constants::messages};
use actix_web::{Error as ActixError, FromRequest, dev::Payload, web};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use futures::future::{Ready, ready};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)
        .map(|ph| ph.to_string())
}

pub fn verify_password(hash: &str, plain: &str) -> Result<(), AppError> {
    let parsed = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized(messages::INVALID_CREDENTIALS.into()))
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
    .map_err(|_| AppError::Unauthorized(messages::AUTH_REQUIRED.into()))
}

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, ActixError>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = req.app_data::<web::Data<AppConfig>>().cloned();
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        ready(match (cfg, auth_header) {
            (Some(cfg), Some(header)) if header.starts_with("Bearer ") => {
                let token = header.trim_start_matches("Bearer ").trim();
                match decode_token(&cfg, token) {
                    Ok(claims) => Ok(AuthenticatedUser {
                        user_id: claims.sub,
                    }),
                    Err(_) => Err(AppError::Unauthorized(messages::AUTH_REQUIRED.into()).into()),
                }
            }
            _ => Err(AppError::Unauthorized(messages::AUTH_REQUIRED.into()).into()),
        })
    }
}
