mod v1;

use actix_web::{web, Scope};

pub(super) fn build_scope() -> Scope {
    web::scope("/api").service(v1::build_scope())
}
