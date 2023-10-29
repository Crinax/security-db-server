use actix_web::{post, Responder, HttpResponse, web::Json};
use validator::Validate;

use crate::services::dto::auth::RegistrationDto;


#[post("auth")]
pub(super) async fn register(json: Json<RegistrationDto>) -> impl Responder {
    match json.validate() {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(err) => HttpResponse::BadRequest().json(err)
    }
}
