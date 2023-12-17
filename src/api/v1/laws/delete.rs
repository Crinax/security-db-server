use actix_web::{delete, HttpRequest, web::{Path, Data}, Responder, HttpMessage, HttpResponse};
use uuid::Uuid;

use crate::{services::auth::JwtAccessData, api::errors::JsonMessage, state::AppState};

#[delete("{uid}")]
fn delete(req: HttpRequest, state: Data<AppState>, uid: Path<Uuid>) -> impl Responder {
    let user = req.extensions().get::<JwtAccessData>().cloned();

    if user.is_none() {
        return HttpResponse::Forbidden().json(JsonMessage {
            message: "token_not_found"
        })
    }

    let user = user.unwrap();

    if user.role != "admin" {
        return HttpResponse::Forbidden().json(JsonMessage {
            message: "no_rights"
        });
    }

    return HttpResponse::Ok().json(JsonMessage {
        message: "ok"
    });
}
