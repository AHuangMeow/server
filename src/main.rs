mod auth;
mod config;
mod constants;
mod database;
mod errors;
mod handlers;
mod models;

use crate::config::AppConfig;
use crate::database::mongodb::{UserRepository, init_db};
use crate::database::redis::{TokenBlacklist, init_redis};
use crate::handlers::{admin_scope, auth_scope, user_scope};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use rustls::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

fn load_rustls_config(
    cert_path: &str,
    key_path: &str,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let cert_file = &mut BufReader::new(File::open(cert_path)?);
    let key_file = &mut BufReader::new(File::open(key_path)?);

    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>()?;
    let mut keys = pkcs8_private_keys(key_file).collect::<Result<Vec<_>, _>>()?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0).into())?;

    Ok(config)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    
    let cfg = AppConfig::from_env().expect("Failed to load configuration");
    let db = init_db(&cfg.mongo_uri, &cfg.mongo_db)
        .await
        .expect("Failed to connect to database");
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
            .app_data(Data::new(cfg.clone()))
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(blacklist.clone()))
            .service(auth_scope())
            .service(user_scope())
            .service(admin_scope())
    });

    let server = match (&ssl_cert_path, &ssl_key_path) {
        (Some(cert_path), Some(key_path)) => {
            println!("Starting HTTPS server at https://{}:{}", host, port);
            let tls_config =
                load_rustls_config(cert_path, key_path).expect("Failed to load SSL certificates");
            server.bind_rustls_0_23((host, port), tls_config)?
        }
        _ => panic!("Missing SSL certificates paths"),
    };

    server.run().await
}
