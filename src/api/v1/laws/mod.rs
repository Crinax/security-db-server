mod get;
mod delete;

use std::sync::Arc;

use crate::config::Config;

use actix_web::web;

pub(super) fn configure(_: Arc<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg| {
        cfg.service(get::get_laws);
    }
}
