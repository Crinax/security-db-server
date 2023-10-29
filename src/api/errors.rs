use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str
}

pub fn invalid_data() -> HttpResponse {
    return HttpResponse::BadRequest().json(JsonMessage { message: "invalid_data" })
}
