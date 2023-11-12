use actix_web::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        Cookie,
    },
    post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use serde::Serialize;
use validator::Validate;

use crate::{
    api::errors::{invalid_data, JsonMessage},
    db::DbError,
    services::{
        auth::AuthServiceError,
        dto::auth::{AuthorizationDto, RegistrationDto},
    },
    state::AppState,
};

#[derive(Serialize)]
struct AuthDataResult {
    access_token: String,
    expires: usize,
}

#[post("register")]
pub(super) async fn register(json: Json<RegistrationDto>, state: Data<AppState>) -> impl Responder {
    if json.validate().is_err() {
        return invalid_data();
    }
    let internal_error = HttpResponse::InternalServerError().json(JsonMessage {
        message: "internal_error",
    });

    let clonned_state = state.clone();
    let block_result =
        web::block(move || state.auth_service().register_user(json.0, state.config())).await;

    if block_result.is_err() {
        return internal_error;
    }

    let service_result = block_result.unwrap();

    if let Err(service_err) = service_result {
        match service_err {
            DbError::Execution(AuthServiceError::AlreadyExists) => {
                return HttpResponse::Conflict().json(JsonMessage {
                    message: "already_exists",
                })
            }
            _ => return internal_error,
        }
    }

    let tokens = service_result.unwrap();

    let _ = clonned_state.redis().add_pair(&tokens.1, "ok", tokens.3);
    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days()))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2
        })
}

#[post("")]
pub(super) async fn authorize(
    json: Json<AuthorizationDto>,
    state: Data<AppState>,
) -> impl Responder {
    if json.validate().is_err() {
        return invalid_data();
    }

    let internal_error = HttpResponse::InternalServerError().json(JsonMessage {
        message: "internal_error",
    });

    let clonned_state = state.clone();
    let block_result =
        web::block(move || state.auth_service().authorize_user(json.0, state.config())).await;

    if block_result.is_err() {
        return internal_error;
    }

    let db_result = block_result.unwrap();

    if let Err(db_err) = db_result {
        match db_err {
            DbError::Execution(service_result) => match service_result {
                AuthServiceError::UserNotFound => return invalid_data(),
                AuthServiceError::InvalidPassword => return invalid_data(),
                _ => return internal_error,
            },
            _ => return internal_error,
        }
    }

    let tokens = db_result.unwrap();
    let _ = clonned_state.redis().add_pair(&tokens.1, "ok", tokens.3);
    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days()))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2
        })
}
