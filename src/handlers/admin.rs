use crate::auth::AdminUser;
use crate::auth::hash_password;
use crate::constants::*;
use crate::errors::AppError;
use crate::models::dto::*;
use crate::models::user::User;
use crate::repository::UserRepository;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Scope, delete, get, post, put};
use mongodb::bson::oid::ObjectId;

#[get("/users")]
async fn get_all_users(
    _admin: AdminUser,
    repo: Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let users = repo.find_all().await?;

    let user_infos: Vec<UserInfo> = users
        .into_iter()
        .map(|u| UserInfo {
            id: u.id.to_hex(),
            email: u.email,
            username: u.username,
            is_admin: u.is_admin,
        })
        .collect();

    Ok(HttpResponse::Ok().json(UserListResponse {
        msg: USER_INFOS_FETCHED.into(),
        data: user_infos,
    }))
}

#[get("/users/{id}")]
async fn get_user_by_id(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    let user = repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    Ok(HttpResponse::Ok().json(UserInfoResponse {
        msg: USER_INFO_FETCHED.into(),
        data: UserInfo {
            id: user.id.to_hex(),
            email: user.email,
            username: user.username,
            is_admin: user.is_admin,
        },
    }))
}

#[post("/users")]
async fn create_user(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    req: Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    if repo.find_by_email(&req.email).await?.is_some() {
        return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
    }

    let password_hash = hash_password(&req.password)?;

    let user = User {
        id: ObjectId::new(),
        email: req.email.clone(),
        username: req.username.clone(),
        password_hash,
        is_admin: req.is_admin,
    };

    repo.create(&user).await?;

    Ok(HttpResponse::Created().json(ResultResponse {
        msg: USER_CREATED.into(),
    }))
}

#[put("/users/{id}")]
async fn update_user(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
    req: Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    let user = repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    if let Some(ref email) = req.email {
        if email != &user.email {
            if repo.find_by_email(email).await?.is_some() {
                return Err(AppError::Conflict(EMAIL_ALREADY_EXISTS.into()));
            }
            repo.update_email(&object_id, email).await?;
        }
    }

    if let Some(ref username) = req.username {
        repo.update_username(&object_id, username).await?;
    }

    if let Some(ref password) = req.password {
        let password_hash = hash_password(password)?;
        repo.update_password(&object_id, &password_hash).await?;
    }

    Ok(HttpResponse::Ok().json(ResultResponse {
        msg: USER_UPDATED.into(),
    }))
}

#[delete("/users/{id}")]
async fn delete_user(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    repo.find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    repo.delete_by_id(&object_id).await?;

    Ok(HttpResponse::Ok().json(ResultResponse {
        msg: USER_DELETED.into(),
    }))
}

#[put("/users/{id}/admin")]
async fn set_admin(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
    req: Json<SetAdminRequest>,
) -> Result<HttpResponse, AppError> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| AppError::BadRequest(INVALID_USER_ID.into()))?;

    repo.find_by_id(&object_id)
        .await?
        .ok_or_else(|| AppError::NotFound(USER_NOT_FOUND.into()))?;

    repo.set_admin(&object_id, req.is_admin).await?;

    let msg = if req.is_admin {
        USER_SETED_AS_ADMIN
    } else {
        ADMIN_SETED_AS_USER
    };

    Ok(HttpResponse::Ok().json(ResultResponse { msg: msg.into() }))
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
