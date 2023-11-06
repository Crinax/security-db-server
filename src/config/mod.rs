use std::env;

use crate::services::auth::{SaltProvider, SecretsProvider};

use super::db::DbUrlProvider;

pub struct Config {
    db_url: String,
    host: String,
    port: u16,
    salt: String,
    jwt_secret_access: String,
    jwt_secret_refresh: String,
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

impl SecretsProvider for Config {
    fn access_secret(&self) -> &[u8] {
        self.jwt_secret_access.as_bytes()
    }

    fn refresh_secret(&self) -> &[u8] {
        self.jwt_secret_refresh.as_bytes()
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
                log::warn!("SALT not specified. Default value is not secure");

                "notsecuresalt".to_string()
            }),
            jwt_secret_access: env::var("JWT_SECRET_ACCESS").unwrap_or_else(|_| {
                log::warn!("JWT_SECRET_ACCESS not specified. Default value is not secure");

                "notsecuresecretaccess".to_string()
            }),
            jwt_secret_refresh: env::var("JWT_SECRET_REFRESH").unwrap_or_else(|_| {
                log::warn!("JWT_SECRET_REFRESH not specified. Default value is not secure");

                "notsecuresecretrefresh".to_string()
            }),
        }
    }
}
