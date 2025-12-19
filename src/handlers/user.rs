use crate::auth::AuthenticatedUser;
use crate::constants::*;
use crate::database::mongodb::UserRepository;
use crate::errors::AppError;
use crate::models::request::{UpdateEmailRequest, UpdatePasswordRequest, UpdateUsernameRequest};
use crate::models::response::{AboutMe, Response};
use crate::utils::password::{hash_password, verify_password};
use actix_web::web::{scope, Data, Json};
use actix_web::{get, put, HttpResponse};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

#[get("/me")]
pub async fn get_me(
    user_repo: Data<UserRepository>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let uid = ObjectId::parse_str(&user.user_id)?;
    let user_doc = user_repo
        .find_by_id(&uid)
        .await?
        .ok_or(AppError::Unauthorized(USER_NOT_FOUND.into()))?;

    Ok(HttpResponse::Ok().json(Response {
        msg: PROFILE_FETCHED.into(),
        data: Some(AboutMe {
            email: user_doc.email,
            username: user_doc.username,
        }),
    }))
}

#[put("/email")]
async fn update_email(
    user_repo: Data<UserRepository>,
    user: AuthenticatedUser,
    payload: Json<UpdateEmailRequest>,
) -> Result<HttpResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    if user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
    }

    let uid = ObjectId::parse_str(&user.user_id)?;
    user_repo.update_email(&uid, &payload.email).await?;

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: EMAIL_UPDATED.into(),
        data: None,
    }))
}

#[put("/username")]
async fn update_username(
    user_repo: Data<UserRepository>,
    user: AuthenticatedUser,
    payload: Json<UpdateUsernameRequest>,
) -> Result<HttpResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let uid = ObjectId::parse_str(&user.user_id)?;
    user_repo.update_username(&uid, &payload.username).await?;

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: USERNAME_UPDATED.into(),
        data: None,
    }))
}

#[put("/password")]
async fn update_password(
    user_repo: Data<UserRepository>,
    user: AuthenticatedUser,
    payload: Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let uid = ObjectId::parse_str(&user.user_id)?;
    let current = user_repo
        .find_by_id(&uid)
        .await?
        .ok_or(AppError::Unauthorized(USER_NOT_FOUND.into()))?;

    verify_password(&current.password_hash, &payload.old_password)
        .map_err(|_| AppError::Unauthorized(INVALID_OLD_PASSWORD.into()))?;

    let new_hash = hash_password(&payload.new_password)?;
    user_repo.update_password(&uid, &new_hash).await?;

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: PASSWORD_UPDATED.into(),
        data: None,
    }))
}

pub fn user_scope() -> actix_web::Scope {
    scope("/user")
        .service(get_me)
        .service(update_email)
        .service(update_username)
        .service(update_password)
}
