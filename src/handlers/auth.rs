use crate::{
    auth::{AuthenticatedUser, generate_token, hash_password, verify_password},
    config::AppConfig,
    errors::AppError,
    models::dto::{AuthResponse, LoginRequest, RegisterRequest},
};
use actix_web::{HttpResponse, post, web};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId},
};

#[post("/auth/register")]
async fn register(
    db: web::Data<Database>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    // 唯一性检查
    let users = db.collection::<mongodb::bson::Document>("users");
    if users
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|_| AppError::Internal)?
        .is_some()
    {
        return Err(AppError::BadRequest("Email exists".into()));
    }
    if users
        .find_one(doc! { "username": &payload.username })
        .await
        .map_err(|_| AppError::Internal)?
        .is_some()
    {
        return Err(AppError::BadRequest("Username exists".into()));
    }

    let hash = hash_password(&payload.password)?;
    let now = DateTime::now();
    let user_id = ObjectId::new();
    users
        .insert_one(doc! {
            "_id": &user_id,
            "email": &payload.email,
            "username": &payload.username,
            "password_hash": hash,
            "created_at": now,
            "updated_at": now,
        })
        .await
        .map_err(|_| AppError::Internal)?;

    let token = generate_token(&cfg, &user_id.to_hex())?;
    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[post("/auth/login")]
async fn login(
    db: web::Data<Database>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    let user = users
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Unauthorized)?;

    let hash: String = user
        .get_str("password_hash")
        .map_err(|_| AppError::Internal)?
        .into();
    verify_password(&hash, &payload.password)?;

    let id = user
        .get_object_id("_id")
        .map_err(|_| AppError::Internal)?
        .to_hex();
    let token = generate_token(&cfg, &id)?;
    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[post("/auth/logout")]
async fn logout(_user: AuthenticatedUser) -> Result<HttpResponse, AppError> {
    // 无状态 JWT: 前端删除 token；如需黑名单可在 DB 记录失效 token 或 jti
    Ok(HttpResponse::Ok().finish())
}

pub fn auth_scope() -> actix_web::Scope {
    web::scope("")
        .service(register)
        .service(login)
        .service(logout)
}
