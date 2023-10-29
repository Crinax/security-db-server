use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct JsonError<'a> {
    pub message: &'a str
}

pub fn invalid_data() -> HttpResponse {
    return HttpResponse::BadRequest().json(JsonError { message: "invalid_data" })
}
