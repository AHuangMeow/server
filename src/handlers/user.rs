use crate::{
    auth::{AuthenticatedUser, hash_password, verify_password},
    errors::AppError,
    models::dto::{
        GetMeResponse, ResultResponse, UpdateEmailRequest, UpdatePasswordRequest,
        UpdateUsernameRequest, UserProfile,
    },
};
use actix_web::{HttpResponse, get, put, web};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId},
};

#[get("/me")]
pub async fn get_me(
    db: web::Data<Database>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    let uid = ObjectId::parse_str(&user.user_id)
        .map_err(|_| AppError::Unauthorized("failed to parse user id".into()))?;
    let doc = users
        .find_one(doc! { "_id": uid })
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Unauthorized("failed to find a user".into()))?;

    let email = doc
        .get_str("email")
        .map_err(|_| AppError::Internal)?
        .to_string();
    let username = doc
        .get_str("username")
        .map_err(|_| AppError::Internal)?
        .to_string();

    Ok(HttpResponse::Ok().json(GetMeResponse {
        code: 200,
        msg: "successfully fetched user profile".into(),
        data: UserProfile { email, username },
    }))
}

#[put("/email")]
async fn update_email(
    db: web::Data<Database>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateEmailRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    // 唯一性校验
    if users
        .find_one(doc! { "email": &payload.new_email })
        .await
        .map_err(|_| AppError::Internal)?
        .is_some()
    {
        return Err(AppError::Conflict("email already registered".into()));
    }
    users
        .update_one(
            doc! { "_id": ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized("failed to parse user id".into()))? },
            doc! { "$set": { "email": &payload.new_email, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: "successfully updated email".into(),
    }))
}

#[put("/username")]
async fn update_username(
    db: web::Data<Database>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateUsernameRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");

    users
        .update_one(
            doc! { "_id": ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized("failed to parse user id".into()))? },
            doc! { "$set": { "username": &payload.new_username, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: "successfully updated username".into(),
    }))
}

#[put("/password")]
async fn update_password(
    db: web::Data<Database>,
    user: AuthenticatedUser,
    payload: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    let uid = ObjectId::parse_str(&user.user_id)
        .map_err(|_| AppError::Unauthorized("failed to parse user id".into()))?;
    let current = users
        .find_one(doc! { "_id": &uid })
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Unauthorized("failed to find a user".into()))?;
    let hash: String = current
        .get_str("password_hash")
        .map_err(|_| AppError::Internal)?
        .into();
    verify_password(&hash, &payload.old_password)
        .map_err(|_| AppError::Unauthorized("invalid old password".into()))?;
    if payload.new_password.len() < 8 {
        return Err(AppError::UnprocessableEntity(
            "password length must be at least 8".into(),
        ));
    }
    let new_hash = hash_password(&payload.new_password)?;
    users
        .update_one(
            doc! { "_id": uid },
            doc! { "$set": { "password_hash": new_hash, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: "successfully updated password".into(),
    }))
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(get_me)
        .service(update_email)
        .service(update_username)
        .service(update_password)
}
