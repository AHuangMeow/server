mod auth;
mod config;
mod constants;
mod db;
mod errors;
mod handlers;
mod models;
mod repository;

use crate::{
    config::AppConfig,
    db::init_db,
    handlers::{auth::auth_scope, user::user_scope},
    repository::UserRepository,
};
use actix_web::{App, HttpServer, web::Data};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let cfg = AppConfig::from_env().expect("Failed to load configuration");
    let db = init_db(&cfg.mongo_uri, &cfg.mongo_db)
        .await
        .expect("Failed to connect to database");
    let user_repo = UserRepository::new(&db);

    let host = cfg.host.clone();
    let port = cfg.port;

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cfg.clone()))
            .app_data(Data::new(user_repo.clone()))
            .service(auth_scope())
            .service(user_scope())
    })
    .bind((host, port))?
    .run()
    .await
}
