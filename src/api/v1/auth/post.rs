use actix_web::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        Cookie,
    },
    post,
    web::{self, Data, Json},
    HttpResponse, Responder, HttpRequest,
};
use serde::Serialize;
use validator::Validate;

use crate::{
    api::errors::{invalid_data, JsonMessage},
    db::DbError,
    services::{
        auth::{AuthServiceError, AuthService},
        dto::auth::{AuthorizationDto, RegistrationDto},
    },
    state::AppState,
};

#[derive(Serialize)]
struct AuthDataResult {
    access_token: String,
    expires: usize,
}

#[post("logout")]
pub(super) async fn logout(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let refresh_token = req.cookie("refresh_token");

    if refresh_token.is_none() {
        return HttpResponse::Ok()
            .json(JsonMessage {
                message: "already_removed",
            });
    }

    let refresh_token = refresh_token.unwrap().to_string();
    let _ = state.redis().remove(&refresh_token);
    let expires_time = OffsetDateTime::from_unix_timestamp(0);
    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", &refresh_token)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() - 30.days()))
                .finish(),
        )
        .json(JsonMessage {
            message: "ok",
        })
}

#[post("refresh-tokens")]
pub(super) async fn refresh_tokens(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let refresh_token = req.cookie("refresh_token");
    let refresh_token_not_found = HttpResponse::Unauthorized().json(JsonMessage {
        message: "refresh_token_not_found",
    });
    let internal_error = HttpResponse::InternalServerError().json(JsonMessage {
        message: "internal_error",
    });
    let clonned_state = state.clone();

    if refresh_token.is_none() {
        return refresh_token_not_found;
    }

    let refresh_token = refresh_token.unwrap();
    let refresh_token = refresh_token.value();
    let access_token = clonned_state.redis().get_pair(&refresh_token);

    if access_token.is_err() {
        return internal_error;
    }

    let access_token = access_token.unwrap();

    if access_token.is_none() {
        return refresh_token_not_found;
    }

    let access_token = access_token.unwrap();

    if refresh_token.is_empty() {
        return refresh_token_not_found;
    }

    let user_data = AuthService::decrypt_token(&access_token, state.config());

    if let Err(err) = user_data {
        match err {
            AuthServiceError::InvalidToken => return HttpResponse::BadRequest().json(JsonMessage {
                message: "invalid_token"
            }),
            _ => return internal_error,
        }
    }

    let user_data = user_data.unwrap();

    let block_result =
        web::block(move || state.auth_service().refresh_tokens(&user_data, state.config())).await;

    if block_result.is_err() {
        return internal_error;
    }

    let service_result = block_result.unwrap();

    if let Err(err) = service_result {
        match err {
            DbError::Execution(AuthServiceError::UserNotFound) =>
                return HttpResponse::NotFound().json(JsonMessage {
                    message: "user_not_found"
                }),
            _ => return internal_error,
        }
    }

    let tokens = service_result.unwrap();

    let _ = clonned_state.redis().remove(&refresh_token);
    let _ = clonned_state.redis().add_pair(&tokens.1, &access_token, tokens.3);

    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64 * 1000);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days() * 1000))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2 * 1000
        })
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

    let _ = clonned_state.redis().add_pair(&tokens.1, &tokens.0, tokens.3);
    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64 * 1000);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days() * 1000))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2 * 1000
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
    let _ = clonned_state.redis().add_pair(&tokens.1, &tokens.0, tokens.3);
    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64 * 1000);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days() * 1000))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2
        })
}
