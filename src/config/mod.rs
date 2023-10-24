use std::env;

use super::accessors::DbUrlProvider;

pub struct Config {
    db_url: String,
    host: String,
    port: u16
}

impl Config {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl DbUrlProvider for Config {
    fn db_url(&self) -> &str {
        &self.db_url
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or("127.0.0.1".into()),
            port: env::var("PORT").map(|e| e.parse().unwrap_or(7878)).unwrap_or(7878)
        }
    }
}
