mod auth;
mod config;
mod constants;
mod database;
mod errors;
mod handlers;
mod models;
mod utils;

use crate::config::app_config::AppConfig;
use crate::config::rustls_config::load_rustls_config;
use crate::database::mongodb::{init_mongodb, UserRepository};
use crate::database::redis::{init_redis, TokenBlacklist};
use crate::handlers::{admin_scope, auth_scope, health_check, user_scope};
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing::info!("Loading configuration...");
    let cfg = AppConfig::from_env().expect("Failed to load configuration");

    tracing::info!("Connecting to MongoDB at {}...", cfg.mongo_uri);
    let db = init_mongodb(&cfg.mongo_uri, &cfg.mongo_db)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connecting to Redis at {}...", cfg.redis_uri);
    let redis_conn = init_redis(&cfg.redis_uri)
        .await
        .expect("Failed to connect to Redis");

    let user_repo = UserRepository::new(&db);
    let blacklist = TokenBlacklist::new(redis_conn);

    let host = cfg.host.clone();
    let port = cfg.port;
    let ssl_cert_path = cfg.ssl_cert_path.clone();
    let ssl_key_path = cfg.ssl_key_path.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(Data::new(cfg.clone()))
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(blacklist.clone()))
            .service(health_check)
            .service(auth_scope())
            .service(user_scope())
            .service(admin_scope())
    });

    let server = match (&ssl_cert_path, &ssl_key_path) {
        (Some(cert_path), Some(key_path)) => {
            tracing::info!("Starting HTTPS server at https://{}:{}", host, port);
            let tls_config =
                load_rustls_config(cert_path, key_path).expect("Failed to load SSL certificates");
            server.bind_rustls_0_23((host, port), tls_config)?
        }
        _ => {
            tracing::warn!("SSL certificates not configured, falling back to HTTP");
            tracing::info!("Starting HTTP server at http://{}:{}", host, port);
            server.bind((host, port))?
        }
    };

    server.run().await
}
