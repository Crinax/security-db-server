use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::errors::{invalid_data, JsonMessage},
    services::dto::auth::RegistrationDto,
    state::AppState,
};

#[post("register")]
pub(super) async fn register(json: Json<RegistrationDto>, state: Data<AppState>) -> impl Responder {
    match json.validate() {
        Ok(_) => {
            match web::block(move || state.auth_service().register_user(json.0, state.config()))
                .await
            {
                Ok(_) => HttpResponse::Ok().json(JsonMessage { message: "ok" }),
                Err(_) => HttpResponse::InternalServerError().json(JsonMessage {
                    message: "server error",
                }),
            }
        }
        Err(_) => invalid_data(),
    }
}
