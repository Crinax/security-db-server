use std::env;

use crate::services::auth::SaltProvider;

use super::db::DbUrlProvider;

pub struct Config {
    db_url: String,
    host: String,
    port: u16,
    salt: String,
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

impl SaltProvider for Config {
    fn salt(&self) -> &[u8] {
        self.salt.as_bytes()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or("127.0.0.1".into()),
            port: env::var("PORT")
                .map(|e| e.parse().unwrap_or(7878))
                .unwrap_or(7878),
            salt: env::var("SALT").unwrap_or_else(|_| {
                log::warn!("It is not secure to use default salt. Please specify own salt");

                "notsecuresalt".to_string()
            }),
        }
    }
}
