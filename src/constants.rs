pub const COLLECTION_USERS: &str = "users";

pub const MIN_PASSWORD_LENGTH: usize = 8;

pub const DEFAULT_JWT_EXP_HOURS: i64 = 24;
pub const MIN_JWT_SECRET_LENGTH: usize = 32;

pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: &str = "8080";

pub const REGISTER_SUCCESS: &str = "successfully registered";
pub const LOGIN_SUCCESS: &str = "successfully logged in";
pub const LOGOUT_SUCCESS: &str = "successfully logged out";
pub const PROFILE_FETCHED: &str = "successfully fetched user profile";
pub const EMAIL_UPDATED: &str = "successfully updated email";
pub const USERNAME_UPDATED: &str = "successfully updated username";
pub const PASSWORD_UPDATED: &str = "successfully updated password";

pub const EMAIL_ALREADY_EXISTS: &str = "email already registered";
pub const INVALID_CREDENTIALS: &str = "invalid username or password";
pub const INVALID_OLD_PASSWORD: &str = "invalid old password";
pub const USER_NOT_FOUND: &str = "user not found";
pub const AUTH_REQUIRED: &str = "authentication required";
pub const INVALID_USER_ID: &str = "invalid user id";
pub const PASSWORD_TOO_SHORT: &str = "password length must be at least 8";
pub const PERMISSION_DENIED: &str = "permission denied";

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_DB: &str = "MONGO_DB";
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const JWT_EXP_HOURS: &str = "JWT_EXP_HOURS";
pub const APP_HOST: &str = "APP_HOST";
pub const APP_PORT: &str = "APP_PORT";
