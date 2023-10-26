pub mod router;

use actix_web::{web, Scope};

pub fn build_scope() -> Scope {
    web::scope("/v1").service(router::laws::build_scope())
}
