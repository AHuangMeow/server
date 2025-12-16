use crate::{
    auth::{generate_token, hash_password, verify_password, AuthenticatedUser},
    config::AppConfig,
    constants::{messages, MIN_PASSWORD_LENGTH},
    errors::AppError,
    models::{dto::{AuthResponse, LoginRequest, RegisterRequest, ResultResponse}, user::User},
    repository::UserRepository,
};
use actix_web::{HttpResponse, post, web};
use mongodb::bson::{DateTime, oid::ObjectId};

#[post("/register")]
async fn register(
    user_repo: web::Data<UserRepository>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if user_repo
        .find_by_email(&payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(messages::EMAIL_ALREADY_EXISTS.into()));
    }

    if payload.password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::UnprocessableEntity(
            messages::PASSWORD_TOO_SHORT.into(),
        ));
    }

    let hash = hash_password(&payload.password)?;
    let now = DateTime::now();
    let user_id = ObjectId::new();
    let new_user = User {
        id: user_id,
        email: payload.email.clone(),
        username: payload.username.clone(),
        password_hash: hash,
        created_at: now,
        updated_at: now,
    };
    user_repo.create(&new_user).await?;

    let token = generate_token(&cfg, &user_id.to_hex())?;
    Ok(HttpResponse::Ok().json(AuthResponse {
        code: 200,
        msg: messages::REGISTER_SUCCESS.into(),
        token,
    }))
}

#[post("/login")]
async fn login(
    user_repo: web::Data<UserRepository>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = user_repo
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::Unauthorized(
            messages::INVALID_CREDENTIALS.into(),
        ))?;

    verify_password(&user.password_hash, &payload.password)?;

    let id = user.id.to_hex();
    let token = generate_token(&cfg, &id)?;
    Ok(HttpResponse::Ok().json(AuthResponse {
        code: 200,
        msg: messages::LOGIN_SUCCESS.into(),
        token,
    }))
}

#[post("/logout")]
async fn logout(_user: AuthenticatedUser) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: messages::LOGOUT_SUCCESS.into(),
    }))
}

pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
        .service(logout)
}
