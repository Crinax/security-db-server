pub mod router;

use actix_web::{web, Scope};

pub fn make_service() -> Scope {
    web::scope("/v1")
        .service(router::laws::make_service())
}
