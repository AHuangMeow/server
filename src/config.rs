use crate::constants::*;
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub jwt_secret: String,
    pub jwt_exp_hours: i64,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        let jwt_secret = env::var(JWT_SECRET).map_err(|_| format!("{} is required", JWT_SECRET))?;

        if jwt_secret.len() < MIN_JWT_SECRET_LENGTH {
            return Err(format!(
                "{} must be at least {} characters",
                JWT_SECRET, MIN_JWT_SECRET_LENGTH,
            ));
        }

        let mongo_uri = env::var(MONGO_URI).map_err(|_| format!("{} is required", MONGO_URI))?;

        let mongo_db = env::var(MONGO_DB).map_err(|_| format!("{} is required", MONGO_DB))?;

        let jwt_exp_hours = env::var(JWT_EXP_HOURS)
            .unwrap_or_else(|_| DEFAULT_JWT_EXP_HOURS.to_string())
            .parse()
            .map_err(|_| format!("{} must be a valid number", JWT_EXP_HOURS))?;

        if jwt_exp_hours <= 0 {
            return Err(format!("{} must be positive", JWT_EXP_HOURS));
        }

        let host = env::var(APP_HOST).unwrap_or_else(|_| DEFAULT_HOST.into());

        let port = env::var(APP_PORT)
            .unwrap_or_else(|_| DEFAULT_PORT.into())
            .parse()
            .map_err(|_| format!("{} must be a valid port number", APP_PORT))?;

        Ok(Self {
            mongo_uri,
            mongo_db,
            jwt_secret,
            jwt_exp_hours,
            host,
            port,
        })
    }
}
