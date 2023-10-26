mod get;

use actix_web::{web, Scope};
use super::ScopeBuilder;

pub(super) struct LawsScope;

impl ScopeBuilder for LawsScope {
    fn build_scope() -> Scope {
        web::scope("/laws").service(get::get_laws)
    }
}
