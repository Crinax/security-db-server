pub mod errors;
mod middlewares;
mod v1;

use std::sync::Arc;

use actix_web::web;

use crate::config::Config;

pub(super) fn configure(config: Arc<Config>) -> impl Fn(&mut web::ServiceConfig) -> () {
    move |cfg| {
        cfg.service(
            web::scope("/v1")
                .configure(v1::configure(config.clone()))
        );
    }
}
