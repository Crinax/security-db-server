use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::env;

pub struct Config {
    db_url: Arc<str>,
    host: Arc<str>,
    port: u16
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn get_address(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::from_str(self.host()).unwrap(), self.port())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set").into(),
            host: env::var("HOST").unwrap_or("127.0.0.1".into()).into(),
            port: env::var("PORT").map(|e| e.parse().unwrap_or(7878)).unwrap_or(7878)
        }
    }
}
