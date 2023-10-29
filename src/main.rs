mod api;
mod config;
mod db;
mod state;
mod services;

use std::sync::Arc;

use dotenvy::dotenv;

use actix_web::{middleware::Logger, web, App, HttpServer, error};
use config::Config;
use db::{Db, DbProvider, DbUrlProvider};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use env_logger::Env;
use state::AppState;
use api::{ApiScope, ScopeBuilder, errors::invalid_data};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Arc::new(Config::default());
    let db = Arc::new(Db::new(config.db_url()).expect("Db instance error"));

    log::info!("Running migrations...");

    db.migrate(MIGRATIONS).expect("Error while migration");
    let data = web::Data::new(AppState::new(db));
    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            log::error!("{:?}", err);
            error::InternalError::from_response(
                err,
                invalid_data().into()
            ).into()
        });

    log::info!("Starting server at {}:{}", config.host(), config.port());

    HttpServer::new(move || {
        App::new()
            .app_data(json_cfg.clone())
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(ApiScope::build_scope())
    })
    .bind((config.host(), config.port()))?
    .run()
    .await
}
