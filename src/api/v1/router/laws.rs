use actix_web::{web, get, Responder, Scope, HttpResponse};

#[get("")]
pub async fn get_laws() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn build_scope() -> Scope {
    web::scope("/laws")
        .service(get_laws)
}
