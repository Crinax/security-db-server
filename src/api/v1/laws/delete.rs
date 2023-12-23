use actix_web::{delete, HttpRequest, web::{Data, Json, self}, Responder, HttpMessage, HttpResponse};

use crate::{services::{auth::JwtAccessData, dto::user::DeleteLawsRequestResponse}, api::errors::JsonMessage, state::AppState};

#[delete("")]
async fn delete(req: HttpRequest, json: Json<DeleteLawsRequestResponse>, state: Data<AppState>) -> impl Responder {
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

    let uids = json.uids.clone();
    let result = web::block(move || state.user_service().delete_laws(&json.uids)).await;

    if result.is_err() {
        return HttpResponse::InternalServerError().json(JsonMessage {
            message: "internal_error",
        });
    }

    if result.unwrap().is_err() {
        return HttpResponse::InternalServerError().json(JsonMessage {
            message: "internal_error",
        });
    }

    return HttpResponse::Ok().json(DeleteLawsRequestResponse {
        uids
    });
}
