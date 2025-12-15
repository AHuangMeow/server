use crate::{
    auth::{AuthenticatedUser, generate_token, hash_password, verify_password},
    config::AppConfig,
    errors::AppError,
    models::dto::{AuthResponse, LoginRequest, RegisterRequest, ResultResponse},
};
use actix_web::{HttpResponse, post, web};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId},
};

#[post("/register")]
async fn register(
    db: web::Data<Database>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let users = db.collection::<mongodb::bson::Document>("users");
    if users
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|_| AppError::Internal)?
        .is_some()
    {
        return Err(AppError::Conflict("email already registered".into()));
    }

    if payload.password.len() < 8 {
        return Err(AppError::UnprocessableEntity(
            "password length must be at least 8".into(),
        ));
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
    Ok(HttpResponse::Ok().json(AuthResponse {
        code: 200,
        msg: "successfully registered".into(),
        token,
    }))
}

#[post("/login")]
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
        .ok_or(AppError::Unauthorized(
            "invalid username or password".into(),
        ))?;

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
    Ok(HttpResponse::Ok().json(AuthResponse {
        code: 200,
        msg: "successfully logged in".into(),
        token,
    }))
}

#[post("/logout")]
async fn logout(_user: AuthenticatedUser) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(ResultResponse {
        code: 200,
        msg: "successfully logged out".into(),
    }))
}

pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
        .service(logout)
}
