use std::sync::Arc;

use crate::{
    config::Config,
    services::{auth::AuthService, user::UserService}, cache::Cache,
};

pub struct AppState {
    auth_service: AuthService,
    user_service: UserService,
    config: Arc<Config>,
    redis: Cache,
}

impl AppState {
    pub fn new(auth_service: AuthService, user_service: UserService, config: Arc<Config>, redis: Cache) -> Self {
        Self {
            auth_service,
            user_service,
            config,
            redis,
        }
    }

    pub fn auth_service(&self) -> &AuthService {
        &self.auth_service
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn redis(&self) -> &Cache {
        &self.redis
    }
}
