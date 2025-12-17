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

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
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

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cfg.clone()))
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(blacklist.clone()))
            .service(auth_scope())
            .service(user_scope())
            .service(admin_scope())
    })
    .bind((host, port))?
    .run()
    .await
}
