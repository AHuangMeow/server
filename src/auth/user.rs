use crate::auth::token::decode_token;
use crate::config::AppConfig;
use crate::constants::AUTH_REQUIRED;
use crate::errors::AppError;
use actix_web::dev::Payload;
use actix_web::{Error as ActixError, FromRequest, web};
use std::future::{Ready, ready};

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
                    Err(_) => Err(AppError::Unauthorized(AUTH_REQUIRED.into()).into()),
                }
            }
            _ => Err(AppError::Unauthorized(AUTH_REQUIRED.into()).into()),
        })
    }
}
