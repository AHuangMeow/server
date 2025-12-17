use crate::auth::{AuthenticatedUser, generate_token, hash_password, verify_password};
use crate::config::AppConfig;
use crate::constants::*;
use crate::database::mongodb::UserRepository;
use crate::database::redis::TokenBlacklist;
use crate::errors::AppError;
use crate::models::request::{LoginRequest, RegisterRequest};
use crate::models::response::{Response, Token};
use crate::models::user::User;
use actix_web::web::{Data, Json, scope};
use actix_web::{HttpResponse, post};
use mongodb::bson::oid::ObjectId;
use time::OffsetDateTime;

#[post("/register")]
async fn register(
    user_repo: Data<UserRepository>,
    cfg: Data<AppConfig>,
    payload: Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
    }

    if payload.password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::UnprocessableEntity(PASSWORD_TOO_SHORT.into()));
    }

    let hash = hash_password(&payload.password)?;
    let user_id = ObjectId::new();
    let new_user = User {
        id: user_id,
        email: payload.email.clone(),
        username: payload.username.clone(),
        password_hash: hash,
        is_admin: false,
    };
    user_repo.create(&new_user).await?;

    let token = generate_token(&cfg, &user_id.to_hex())?;
    Ok(HttpResponse::Ok().json(Response {
        msg: REGISTER_SUCCESS.into(),
        data: Some(Token { token }),
    }))
}

#[post("/login")]
async fn login(
    user_repo: Data<UserRepository>,
    cfg: Data<AppConfig>,
    payload: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = user_repo
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::Unauthorized(INVALID_CREDENTIALS.into()))?;

    verify_password(&user.password_hash, &payload.password)?;

    let id = user.id.to_hex();
    let token = generate_token(&cfg, &id)?;
    Ok(HttpResponse::Ok().json(Response {
        msg: LOGIN_SUCCESS.into(),
        data: Some(Token { token }),
    }))
}

#[post("/logout")]
async fn logout(
    user: AuthenticatedUser,
    blacklist: Data<TokenBlacklist>,
) -> Result<HttpResponse, AppError> {
    let token = &user.token;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp_seconds = (user.exp as i64) - now;

    if exp_seconds > 0 {
        blacklist.add_token(token, exp_seconds).await?;
    }

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: LOGOUT_SUCCESS.into(),
        data: None,
    }))
}

pub fn auth_scope() -> actix_web::Scope {
    scope("/auth")
        .service(register)
        .service(login)
        .service(logout)
}
