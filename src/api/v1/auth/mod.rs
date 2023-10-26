mod post;

use super::ScopeBuilder;
use actix_web::{web, Scope};

pub(super) struct AuthScope;

impl ScopeBuilder for AuthScope {
    fn build_scope() -> Scope {
        web::scope("/auth")
    }
}
