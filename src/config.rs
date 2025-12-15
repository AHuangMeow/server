use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub jwt_secret: String,
    pub jwt_exp_hours: i64,
}

pub fn load_config() -> (AppConfig, String, u16) {
    dotenv().ok();
    (
        AppConfig {
            mongo_uri: env::var("MONGO_URI").expect("MONGO_URI missing"),
            mongo_db: env::var("MONGO_DB").expect("MONGO_DB missing"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET missing"),
            jwt_exp_hours: env::var("JWT_EXP_HOURS")
                .unwrap_or_else(|_| "24".into())
                .parse()
                .unwrap(),
        },
        env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
        env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .unwrap(),
    )
}
