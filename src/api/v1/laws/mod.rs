mod get;

use super::ScopeBuilder;
use actix_web::{web, Scope};

pub(super) struct LawsScope;

impl ScopeBuilder for LawsScope {
    fn build_scope() -> Scope {
        web::scope("/laws").service(get::get_laws)
    }
}
