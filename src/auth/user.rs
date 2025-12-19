use crate::config::app_config::AppConfig;
use crate::constants::{AUTH_REQUIRED, TOKEN_BLACKLISTED};
use crate::database::redis::TokenBlacklist;
use crate::errors::AppError;
use crate::utils::token::decode_token;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub token: String,
    pub exp: usize,
}

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, ActixError>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = req.app_data::<Data<AppConfig>>().cloned();
        let blacklist = req.app_data::<Data<TokenBlacklist>>().cloned();
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| h.trim_start_matches("Bearer ").trim().to_string());

        Box::pin(async move {
            let cfg = cfg.ok_or(AppError::Internal)?;
            let token = token.ok_or(AppError::Unauthorized(AUTH_REQUIRED.into()))?;
            let claims = decode_token(&cfg, &token)?;

            if let Some(bl) = blacklist {
                if bl.is_blacklisted(&token).await? {
                    return Err(AppError::Unauthorized(TOKEN_BLACKLISTED.into()).into());
                }
            }

            Ok(AuthenticatedUser {
                user_id: claims.sub,
                token,
                exp: claims.exp,
            })
        })
    }
}
