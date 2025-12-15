use crate::{
    auth::{AuthenticatedUser, hash_password, verify_password},
    errors::AppError,
    models::dto::{UpdateEmailRequest, UpdatePasswordRequest, UpdateUsernameRequest, UserProfile},
};
use actix_web::{HttpResponse, get, put, web};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId},
};

#[get("/user/me")]
pub async fn get_me(
    db: web::Data<Database>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    let uid = ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized)?;
    let doc = users
        .find_one(doc! { "_id": uid })
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Unauthorized)?;

    let email = doc
        .get_str("email")
        .map_err(|_| AppError::Internal)?
        .to_string();
    let username = doc
        .get_str("username")
        .map_err(|_| AppError::Internal)?
        .to_string();

    Ok(HttpResponse::Ok().json(UserProfile { email, username }))
}

#[put("/user/email")]
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
        return Err(AppError::BadRequest("Email exists".into()));
    }
    users
        .update_one(
            doc! { "_id": ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized)? },
            doc! { "$set": { "email": &payload.new_email, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/user/username")]
async fn update_username(
    db: web::Data<Database>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateUsernameRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    if users
        .find_one(doc! { "username": &payload.new_username })
        .await
        .map_err(|_| AppError::Internal)?
        .is_some()
    {
        return Err(AppError::BadRequest("Username exists".into()));
    }
    users
        .update_one(
            doc! { "_id": ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized)? },
            doc! { "$set": { "username": &payload.new_username, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/user/password")]
async fn update_password(
    db: web::Data<Database>,
    user: AuthenticatedUser,
    payload: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    let uid = ObjectId::parse_str(&user.user_id).map_err(|_| AppError::Unauthorized)?;
    let current = users
        .find_one(doc! { "_id": &uid })
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Unauthorized)?;
    let hash: String = current
        .get_str("password_hash")
        .map_err(|_| AppError::Internal)?
        .into();
    verify_password(&hash, &payload.old_password)?;
    let new_hash = hash_password(&payload.new_password)?;
    users
        .update_one(
            doc! { "_id": uid },
            doc! { "$set": { "password_hash": new_hash, "updated_at": DateTime::now() } },
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(HttpResponse::Ok().finish())
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("")
        .service(get_me)
        .service(update_email)
        .service(update_username)
        .service(update_password)
}
