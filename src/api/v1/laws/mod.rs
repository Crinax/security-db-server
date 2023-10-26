mod get;

use actix_web::{web, Scope};

pub(super) fn build_scope() -> Scope {
    web::scope("/laws").service(get::get_laws)
}
