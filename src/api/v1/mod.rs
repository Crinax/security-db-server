mod auth;
mod laws;

use std::sync::Arc;
use actix_web::web;
use crate::config::Config;

use super::middlewares::authenticate::JwtAuth;

pub(super) fn configure(config: Arc<Config>) -> impl Fn(&mut web::ServiceConfig) -> () {
    move |cfg| {
        cfg.service(
            web::scope("/laws")
                .wrap(JwtAuth::new(config.clone()))
                .configure(laws::configure(config.clone()))
        )
        .service(
            web::scope("/auth")
                .configure(auth::configure(config.clone()))
        );
    }
}
