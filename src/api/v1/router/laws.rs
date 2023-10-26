use actix_web::{get, web, HttpResponse, Responder, Scope};

#[get("")]
pub async fn get_laws() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn build_scope() -> Scope {
    web::scope("/laws").service(get_laws)
}
