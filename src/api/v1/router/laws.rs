use actix_web::{web, get, Responder, Scope, HttpResponse};

#[get("")]
pub async fn get_laws() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

pub fn make_service() -> Scope {
    web::scope("/laws")
        .service(get_laws)
}
