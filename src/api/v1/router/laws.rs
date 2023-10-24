use actix_web::{web, get, Responder, Scope, HttpResponse};
use diesel::RunQueryDsl;

use crate::{state::AppState, db::{Db, models::files::File, orm::schema::files::dsl::*}, accessors::DbProvider};

#[get("")]
pub async fn get_laws(app_state: web::Data<AppState<Db>>) -> impl Responder {
    let result = app_state.db().apply(|conn| {
        files.load::<File>(conn)
    });
    HttpResponse::Ok().body(format!("{:?}", result))
}

pub fn build_scope() -> Scope {
    web::scope("/laws")
        .service(get_laws)
}
