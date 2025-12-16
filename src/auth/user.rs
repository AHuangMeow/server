use crate::auth::token::decode_token;
use crate::config::AppConfig;
use crate::constants::AUTH_REQUIRED;
use crate::errors::AppError;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use std::future::{Ready, ready};

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, ActixError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let unauthorized = || {
            ready::<Result<AuthenticatedUser, ActixError>>(Err(AppError::Unauthorized(
                AUTH_REQUIRED.into(),
            )
            .into()))
        };

        let cfg = match req.app_data::<Data<AppConfig>>().cloned() {
            Some(cfg) => cfg,
            None => return ready(Err(AppError::Internal.into())),
        };

        let token = match req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
        {
            Some(h) if h.starts_with("Bearer") => h.trim_start_matches("Bearer").trim(),
            _ => return unauthorized(),
        };

        let claims = match decode_token(&cfg, token) {
            Ok(c) => c,
            Err(_) => return unauthorized(),
        };

        ready(Ok(AuthenticatedUser {
            user_id: claims.sub,
        }))
    }
}
