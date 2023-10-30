use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str,
}

pub fn invalid_data() -> HttpResponse {
    return HttpResponse::BadRequest().json(JsonMessage {
        message: "invalid_data",
    });
}
