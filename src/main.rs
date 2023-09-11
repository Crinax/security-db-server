mod api;

use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new()
           .service(api::make_service())
   }) 
   .bind(("127.0.0.1", 7878))?
   .run()
   .await
}
