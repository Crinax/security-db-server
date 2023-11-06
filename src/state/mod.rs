use std::sync::Arc;

use crate::{
    config::Config,
    services::{auth::AuthService, user::UserService},
};

pub struct AppState {
    auth_service: AuthService,
    user_service: UserService,
    config: Arc<Config>,
}

impl AppState {
    pub fn new(auth_service: AuthService, user_service: UserService, config: Arc<Config>) -> Self {
        Self {
            auth_service,
            user_service,
            config,
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
}
