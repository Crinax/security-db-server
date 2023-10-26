use actix_web::{post, Responder, HttpResponse};


#[post("auth")]
pub(super) async fn pass_authorization() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
