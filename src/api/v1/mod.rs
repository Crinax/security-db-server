mod laws;
mod auth;

use actix_web::{web, Scope};

pub(super) fn build_scope() -> Scope {
    web::scope("/v1").service(laws::build_scope())
}
