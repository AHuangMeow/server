use crate::auth::AdminUser;
use crate::constants::*;
use crate::database::mongodb::UserRepository;
use crate::errors::AppError;
use crate::models::request::{CreateUserRequest, SetRoleRequest, UpdateUserRequest};
use crate::models::response::{Response, UserInfo};
use crate::models::user::User;
use crate::utils::password::hash_password;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse, Scope};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

#[get("/users")]
async fn get_all_users(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let users = user_repo.find_all().await?;

    let user_infos: Vec<UserInfo> = users
        .into_iter()
        .map(|u| UserInfo {
            id: u.id.to_hex(),
            email: u.email,
            username: u.username,
            is_admin: u.is_admin,
        })
        .collect();

    Ok(HttpResponse::Ok().json(Response {
        msg: USER_INFOS_FETCHED.into(),
        data: Some(user_infos),
    }))
}

#[post("/users")]
async fn create_user(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
    payload: Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    if user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
    }

    let password_hash = hash_password(&payload.password)?;

    let user = User {
        id: ObjectId::new(),
        email: payload.email.clone(),
        username: payload.username.clone(),
        password_hash,
        is_admin: payload.is_admin,
        token_version: 0,
    };

    user_repo.create(&user).await?;

    Ok(HttpResponse::Created().json(Response::<()> {
        msg: USER_CREATED.into(),
        data: None,
    }))
}

#[get("/users/{id}")]
async fn get_user_by_id(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    let user = user_repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    Ok(HttpResponse::Ok().json(Response {
        msg: USER_INFO_FETCHED.into(),
        data: Some(UserInfo {
            id: user.id.to_hex(),
            email: user.email,
            username: user.username,
            is_admin: user.is_admin,
        }),
    }))
}

#[put("/users/{id}")]
async fn update_user(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
    id: Path<String>,
    payload: Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    let user = user_repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    if let Some(ref email) = payload.email {
        if email != &user.email {
            if user_repo.find_by_email(email).await?.is_some() {
                return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
            }
            user_repo.update_email(&object_id, email).await?;
        }
    }

    if let Some(ref username) = payload.username {
        user_repo.update_username(&object_id, username).await?;
    }

    if let Some(ref password) = payload.password {
        let password_hash = hash_password(password)?;
        user_repo
            .update_password(&object_id, &password_hash)
            .await?;
    }

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: USER_UPDATED.into(),
        data: None,
    }))
}

#[delete("/users/{id}")]
async fn delete_user(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    user_repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    user_repo.delete_by_id(&object_id).await?;

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: USER_DELETED.into(),
        data: None,
    }))
}

#[put("/users/{id}/admin")]
async fn set_admin(
    _admin: AdminUser,
    user_repo: Data<UserRepository>,
    id: Path<String>,
    payload: Json<SetRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    user_repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    user_repo.set_admin(&object_id, payload.is_admin).await?;

    let msg = if payload.is_admin {
        USER_SET_AS_ADMIN
    } else {
        ADMIN_SET_AS_USER
    };

    Ok(HttpResponse::Ok().json(Response::<()> {
        msg: msg.into(),
        data: None,
    }))
}

pub fn admin_scope() -> Scope {
    Scope::new("/admin")
        .service(get_all_users)
        .service(get_user_by_id)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
        .service(set_admin)
}
