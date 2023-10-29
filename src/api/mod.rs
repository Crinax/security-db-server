pub mod errors;
mod v1;

use actix_web::{web, Scope};
use v1::V1Scope;

pub(super) trait ScopeBuilder {
    fn build_scope() -> Scope;
}

pub(super) struct ApiScope;

impl ScopeBuilder for ApiScope {
    fn build_scope() -> Scope {
        web::scope("/api").service(V1Scope::build_scope())
    }
}
