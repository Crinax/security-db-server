mod api;
mod db;
mod config;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use actix_web::{HttpServer, App, middleware::Logger};
use dotenvy::dotenv;
use env_logger::Env;
use config::Config;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::default();

    // connection.run_pending_migrations(MIGRATIONS)?;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::make_service())
    }) 
    .bind(config.get_address())?
    .run()
    .await
}
