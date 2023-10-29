mod api;
mod config;
mod db;
mod state;
mod services;

use std::sync::Arc;

use dotenvy::dotenv;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use db::{Db, DbProvider, DbUrlProvider};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use env_logger::Env;
use state::AppState;
use api::{ApiScope, ScopeBuilder};

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

    log::info!("Starting server at {}:{}", config.host(), config.port());

    HttpServer::new(move || {
        let clonned = data.clone();
        App::new()
            .app_data(clonned)
            .wrap(Logger::default())
            .service(ApiScope::build_scope())
    })
    .bind((config.host(), config.port()))?
    .run()
    .await
}
