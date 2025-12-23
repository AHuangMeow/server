use crate::config::app_config::AppConfig;
use crate::constants::{AUTH_REQUIRED, PERMISSION_DENIED};
use crate::database::mongodb::UserRepository;
use crate::errors::AppError;
use crate::utils::token::decode_token;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use mongodb::bson::oid::ObjectId;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct AdminUser {}

impl FromRequest for AdminUser {
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, ActixError>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = match req.app_data::<Data<AppConfig>>().cloned() {
            Some(cfg) => cfg,
            None => return Box::pin(async { Err(AppError::Internal.into()) }),
        };

        let repo = match req.app_data::<Data<UserRepository>>().cloned() {
            Some(repo) => repo,
            None => return Box::pin(async { Err(AppError::Internal.into()) }),
        };

        let token = match req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
        {
            Some(h) if h.starts_with("Bearer") => h.trim_start_matches("Bearer").trim(),
            _ => {
                return Box::pin(async {
                    Err(AppError::Unauthorized(AUTH_REQUIRED.into()).into())
                });
            }
        };

        let claims = match decode_token(&cfg, token) {
            Ok(c) => c,
            Err(_) => {
                return Box::pin(async {
                    Err(AppError::Unauthorized(AUTH_REQUIRED.into()).into())
                });
            }
        };

        let user_id = claims.sub.clone();

        Box::pin(async move {
            let object_id = ObjectId::parse_str(&user_id)
                .map_err(|_| AppError::Unauthorized(AUTH_REQUIRED.into()))?;

            let user = repo
                .find_by_id(&object_id)
                .await
                .map_err(|_| AppError::Unauthorized(AUTH_REQUIRED.into()))?
                .ok_or_else(|| AppError::Unauthorized(AUTH_REQUIRED.into()))?;

            if user.token_version != claims.ver {
                return Err(AppError::Unauthorized(AUTH_REQUIRED.into()).into());
            }

            if !user.is_admin {
                return Err(AppError::Forbidden(PERMISSION_DENIED.into()).into());
            }

            Ok(AdminUser {})
        })
    }
}
