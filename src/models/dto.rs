use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmailRequest {
    pub new_email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUsernameRequest {
    pub new_username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub code: usize,
    pub msg: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub code: usize,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct GetMeResponse {
    pub code: usize,
    pub msg: String,
    pub data: UserProfile,
}
