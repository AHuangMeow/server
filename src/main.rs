mod auth;
mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::{
    config::load_config,
    db::init_db,
    handlers::{auth::auth_scope, user::user_scope},
};
use actix_web::{App, HttpServer, web::Data};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let (cfg, host, port) = load_config();
    let db = init_db(&cfg.mongo_uri, &cfg.mongo_db).await.expect("");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cfg.clone()))
            .app_data(Data::new(db.clone()))
            .service(auth_scope())
            .service(user_scope())
    })
    .bind((host, port))?
    .run()
    .await
}
