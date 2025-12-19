use crate::constants::*;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

#[derive(Clone)]
pub struct AppConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub redis_uri: String,
    pub jwt_secret: String,
    pub jwt_exp_hours: i64,
    pub host: String,
    pub port: u16,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
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

        let redis_uri = env::var(REDIS_URI).map_err(|_| format!("{} is required", REDIS_URI))?;

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

        let ssl_cert_path = env::var(SSL_CERT_PATH).ok();
        let ssl_key_path = env::var(SSL_KEY_PATH).ok();

        // Validate SSL certificate files exist
        if let Some(ref cert_path) = ssl_cert_path {
            if !Path::new(cert_path).exists() {
                return Err(format!("SSL certificate file not found: {}", cert_path));
            }
        }

        if let Some(ref key_path) = ssl_key_path {
            if !Path::new(key_path).exists() {
                return Err(format!("SSL key file not found: {}", key_path));
            }
        }

        Ok(Self {
            mongo_uri,
            mongo_db,
            redis_uri,
            jwt_secret,
            jwt_exp_hours,
            host,
            port,
            ssl_cert_path,
            ssl_key_path,
        })
    }
}
