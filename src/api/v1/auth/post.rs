use actix_web::{
    cookie::{time::Duration, Cookie},
    post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use serde::Serialize;
use validator::Validate;

use crate::{
    api::errors::{invalid_data, JsonMessage},
    services::dto::auth::RegistrationDto,
    state::AppState,
};

#[derive(Serialize)]
struct AuthDataResult {
    access_token: String,
}

#[post("register")]
pub(super) async fn register(json: Json<RegistrationDto>, state: Data<AppState>) -> impl Responder {
    if json.validate().is_err() {
        return invalid_data();
    }
    let internal_error = HttpResponse::InternalServerError().json(JsonMessage {
        message: "internal server error",
    });

    let block_result =
        web::block(move || state.auth_service().register_user(json.0, state.config())).await;

    if block_result.is_err() {
        return internal_error;
    }

    let service_result = block_result.unwrap();

    if service_result.is_err() {
        return internal_error;
    }

    let tokens = service_result.unwrap();

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .max_age(Duration::minutes(5))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
        })
}
