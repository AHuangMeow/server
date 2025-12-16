use crate::auth::AdminUser;
use crate::auth::hash_password;
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
) -> Result<HttpResponse, actix_web::Error> {
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
        code: 200,
        msg: "Success".to_string(),
        data: user_infos,
    }))
}

#[get("/users/{id}")]
async fn get_user_by_id(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let user = repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    Ok(HttpResponse::Ok().json(UserInfoResponse {
        code: 200,
        msg: "Success".to_string(),
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
) -> Result<HttpResponse, actix_web::Error> {
    if repo.find_by_email(&req.email).await?.is_some() {
        return Ok(HttpResponse::BadRequest().json(ResultResponse {
            code: 400,
            msg: "Email already exists".to_string(),
        }));
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
        code: 201,
        msg: "User created successfully".to_string(),
    }))
}

#[put("/users/{id}")]
async fn update_user(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
    req: Json<UpdateUserRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let user = repo
        .find_by_id(&object_id)
        .await?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    if let Some(ref email) = req.email {
        if email != &user.email {
            if repo.find_by_email(email).await?.is_some() {
                return Ok(HttpResponse::BadRequest().json(ResultResponse {
                    code: 400,
                    msg: "Email already exists".to_string(),
                }));
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
        code: 200,
        msg: "User updated successfully".to_string(),
    }))
}

#[delete("/users/{id}")]
async fn delete_user(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    repo.find_by_id(&object_id)
        .await?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    repo.delete_by_id(&object_id).await?;

    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: "User deleted successfully".to_string(),
    }))
}

#[put("/users/{id}/admin")]
async fn set_admin(
    _admin: AdminUser,
    repo: Data<UserRepository>,
    id: Path<String>,
    req: Json<SetAdminRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let object_id = ObjectId::parse_str(id.as_str())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    repo.find_by_id(&object_id)
        .await?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    repo.set_admin(&object_id, req.is_admin).await?;

    let msg = if req.is_admin {
        "User set as admin successfully"
    } else {
        "Admin privileges revoked successfully"
    };

    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: msg.to_string(),
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
