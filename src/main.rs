mod api;
mod db;
mod config;

use std::sync::Arc;

use dotenvy::dotenv;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use actix_web::{HttpServer, middleware::Logger, web, App};
use env_logger::Env;
use config::Config;
use db::{Db, DbUrlProvider};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Arc::new(Config::default());
    // connection.run_pending_migrations(MIGRATIONS)?;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Db::new(&config.db_url())))
            .wrap(Logger::default())
            .service(api::make_service())
    }) 
    .bind((config.host(), config.port()))?
    .run()
    .await
}
