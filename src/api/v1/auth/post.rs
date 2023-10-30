use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::errors::{invalid_data, JsonMessage},
    db::Db,
    services::{self, dto::auth::RegistrationDto},
    state::AppState,
};

#[post("register")]
pub(super) async fn register(
    json: Json<RegistrationDto>,
    db: Data<AppState<Db>>,
) -> impl Responder {
    match json.validate() {
        Ok(_) => match web::block(move || {
            let data = json.clone();
            services::auth::register(&db.db(), data.into())
        })
        .await
        {
            Ok(_) => HttpResponse::Ok().json(JsonMessage { message: "ok" }),
            Err(_) => HttpResponse::InternalServerError().json(JsonMessage {
                message: "server error",
            }),
        },
        Err(_) => invalid_data(),
    }
}
