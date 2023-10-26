use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub(super) async fn get_laws() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
