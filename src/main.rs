mod api;
mod db;
mod config;
mod accessors;
mod state;

use std::sync::Arc;

use dotenvy::dotenv;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use actix_web::{HttpServer, middleware::Logger, web, App};
use env_logger::Env;
use config::Config;
use db::Db;
use accessors::{DbUrlProvider, DbProvider};
use state::AppState;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Arc::new(Config::default());
    let clonned_config = config.clone();
    let db = Arc::new(Db::new(clonned_config.db_url()).expect("Db instance error"));

    println!("Migrating...");

    db.migrate(MIGRATIONS).expect("Error while migration");
    let data = web::Data::new(AppState::new(db));

    HttpServer::new(move || {
        let clonned = data.clone();
        App::new()
            .app_data(clonned)
            .wrap(Logger::default())
            .service(api::make_service())
    }) 
    .bind((config.host(), config.port()))?
    .run()
    .await
}
