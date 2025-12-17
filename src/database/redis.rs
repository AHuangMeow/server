use crate::errors::AppError;
use redis::{Client, aio::MultiplexedConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_redis(uri: &str) -> Result<MultiplexedConnection, AppError> {
    let client = Client::open(uri).map_err(|_| AppError::Internal)?;
    client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|_| AppError::Internal)
}

pub struct TokenBlacklist {
    conn: Arc<Mutex<MultiplexedConnection>>,
}

impl TokenBlacklist {
    pub fn new(conn: MultiplexedConnection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub async fn add_token(&self, token: &str, exp_seconds: i64) -> Result<(), AppError> {
        let mut conn = self.conn.lock().await;
        redis::cmd("SETEX")
            .arg(format!("blacklist:{}", token))
            .arg(exp_seconds)
            .arg("1")
            .query_async(&mut *conn)
            .await
            .map_err(|_| AppError::Internal)
    }

    pub async fn is_blacklisted(&self, token: &str) -> Result<bool, AppError> {
        let mut conn = self.conn.lock().await;
        let result: Option<String> = redis::cmd("GET")
            .arg(format!("blacklist:{}", token))
            .query_async(&mut *conn)
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(result.is_some())
    }
}

impl Clone for TokenBlacklist {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
        }
    }
}
