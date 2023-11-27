use actix_web::{get, HttpResponse, Responder, web::{Data, self}};
use serde::Deserialize;

use crate::{state::AppState, db::DbError, api::errors::JsonMessage, services::user::UserServiceError};

#[derive(Deserialize)]
pub struct LawPage {
    pub page: Option<u64>,
}

#[get("")]
pub(super) async fn get_laws(query: web::Query<LawPage>, state: Data<AppState>) -> impl Responder {
    let mut page = 1_u64;

    if query.page.is_some() {
        
        page = query.page.unwrap();
    }

    let internal_error = HttpResponse::InternalServerError().json(
        JsonMessage { message: "internal_error" }
    );
    let unreachable_error = HttpResponse::InternalServerError().json(
        JsonMessage { message: "unreachable_code" }
    );
    let result = web::block(move ||
        state.user_service().get_laws(page)
    )
        .await;

    if result.is_err() {
        return internal_error;
    }

    let unwrapped_result = result.unwrap();

    if let Err(e) = unwrapped_result {
        return match e {
            DbError::Execution(e) => match e {
                UserServiceError::GetLaws => internal_error,
                _ => unreachable_error,
            },
            DbError::Connection => internal_error,
            _ => HttpResponse::InternalServerError().json(
                JsonMessage { message: "unreachable_code" }
            ),
        }
    }

    return HttpResponse::Ok().json(unwrapped_result.unwrap());
}
