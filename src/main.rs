mod api;
mod cache;
mod config;
mod db;
mod services;
mod state;

use std::sync::Arc;

use dotenvy::dotenv;

use crate::services::{auth::AuthService, user::UserService};
use actix_web::{error, middleware::Logger, web, App, HttpServer, http::header::{self, ContentType}};
use api::errors::invalid_data;
use cache::Cache;
use config::Config;
use db::{Db, DbProvider, DbUrlProvider};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use env_logger::Env;
use state::AppState;
use actix_cors::Cors;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Arc::new(Config::default());
    let clonned_config = config.clone();
    let db = Arc::new(Db::new(config.db_url()).expect("Db instance error"));
    let cache = Cache::new(config.redis_url()).expect("Redis instance error");

    log::info!("Running migrations...");

    db.migrate(MIGRATIONS).expect("Error while migration");

    let data = web::Data::new(AppState::new(
        AuthService::new(db.clone()),
        UserService::new(db.clone()),
        config.clone(),
        cache,
    ));

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            log::error!("{:?}", err);
            error::InternalError::from_response(err, invalid_data()).into()
        });

    log::info!("Starting server at {}:{}", config.host(), config.port());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
              .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                header::CONTENT_TYPE
              ])
              .supports_credentials()
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(json_cfg.clone())
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(web::scope("/api").configure(api::configure(clonned_config.clone())))
    })
    .bind((config.host(), config.port()))?
    .run()
    .await
}
