mod api;
mod db;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use actix_web::{HttpServer, App, middleware::Logger};
use dotenvy::dotenv;
use std::env;
use env_logger::Env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connection.run_pending_migrations(MIGRATIONS)?;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::make_service())
    }) 
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
