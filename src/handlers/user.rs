use crate::auth::{AuthenticatedUser, hash_password, verify_password};
use crate::constants::*;
use crate::errors::AppError;
use crate::models::dto::*;
use crate::repository::UserRepository;
use actix_web::{HttpResponse, get, put, web};
use mongodb::bson::oid::ObjectId;

#[get("/me")]
pub async fn get_me(
    user_repo: web::Data<UserRepository>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let uid = ObjectId::parse_str(&user.user_id)?;
    let user_doc = user_repo
        .find_by_id(&uid)
        .await?
        .ok_or(AppError::Unauthorized(USER_NOT_FOUND.into()))?;

    Ok(HttpResponse::Ok().json(GetMeResponse {
        code: 200,
        msg: PROFILE_FETCHED.into(),
        data: UserProfile {
            email: user_doc.email,
            username: user_doc.username,
        },
    }))
}

#[put("/email")]
async fn update_email(
    user_repo: web::Data<UserRepository>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateEmailRequest>,
) -> Result<HttpResponse, AppError> {
    if user_repo.find_by_email(&payload.new_email).await?.is_some() {
        return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
    }

    let uid = ObjectId::parse_str(&user.user_id)?;
    user_repo.update_email(&uid, &payload.new_email).await?;

    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: EMAIL_UPDATED.into(),
    }))
}

#[put("/username")]
async fn update_username(
    user_repo: web::Data<UserRepository>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateUsernameRequest>,
) -> Result<HttpResponse, AppError> {
    let uid = ObjectId::parse_str(&user.user_id)?;
    user_repo
        .update_username(&uid, &payload.new_username)
        .await?;

    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: USERNAME_UPDATED.into(),
    }))
}

#[put("/password")]
async fn update_password(
    user_repo: web::Data<UserRepository>,
    user: AuthenticatedUser,
    payload: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let uid = ObjectId::parse_str(&user.user_id)?;
    let current = user_repo
        .find_by_id(&uid)
        .await?
        .ok_or(AppError::Unauthorized(USER_NOT_FOUND.into()))?;

    verify_password(&current.password_hash, &payload.old_password)
        .map_err(|_| AppError::Unauthorized(INVALID_OLD_PASSWORD.into()))?;

    if payload.new_password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::UnprocessableEntity(PASSWORD_TOO_SHORT.into()));
    }

    let new_hash = hash_password(&payload.new_password)?;
    user_repo.update_password(&uid, &new_hash).await?;

    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: PASSWORD_UPDATED.into(),
    }))
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(get_me)
        .service(update_email)
        .service(update_username)
        .service(update_password)
}
