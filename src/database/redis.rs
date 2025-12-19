use crate::errors::AppError;
use redis::{Client, aio::ConnectionManager};

pub async fn init_redis(uri: &str) -> Result<ConnectionManager, AppError> {
    let client = Client::open(uri).map_err(|_| AppError::Internal)?;
    client
        .get_connection_manager()
        .await
        .map_err(|_| AppError::Internal)
}

#[derive(Clone)]
pub struct TokenBlacklist {
    conn: ConnectionManager,
}

impl TokenBlacklist {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    pub async fn add_token(&self, token: &str, exp_seconds: i64) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        redis::cmd("SETEX")
            .arg(format!("blacklist:{}", token))
            .arg(exp_seconds)
            .arg("1")
            .query_async(&mut conn)
            .await
            .map_err(|_| AppError::Internal)
    }

    pub async fn is_blacklisted(&self, token: &str) -> Result<bool, AppError> {
        let mut conn = self.conn.clone();
        let result: Option<String> = redis::cmd("GET")
            .arg(format!("blacklist:{}", token))
            .query_async(&mut conn)
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(result.is_some())
    }
}
