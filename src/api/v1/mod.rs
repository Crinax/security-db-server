mod auth;
mod laws;

use crate::config::Config;
use actix_web::web;
use std::sync::Arc;

use super::middlewares::authenticate::JwtAuth;

pub(super) fn configure(config: Arc<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(
            web::scope("/laws")
                .wrap(JwtAuth::new(config.clone()))
                .configure(laws::configure(config.clone())),
        )
        .service(web::scope("/auth").configure(auth::configure(config.clone())));
    }
}
