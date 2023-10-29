mod laws;
mod auth;

use actix_web::{web, Scope};

use self::auth::AuthScope;

use super::ScopeBuilder;
use laws::LawsScope;

pub(super) struct V1Scope;

impl ScopeBuilder for V1Scope {
    fn build_scope() -> Scope {
        web::scope("/v1")
            .service(AuthScope::build_scope())
            .service(LawsScope::build_scope())
    }
}
