mod v1;

use actix_web::{web, Scope};

pub fn make_service() -> Scope {
    web::scope("/api")
        .service(v1::make_service())
}
