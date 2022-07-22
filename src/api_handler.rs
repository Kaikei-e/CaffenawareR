mod receive_api_call;

use crate::api_handler::receive_api_call::calculate_decay;
use actix_web::{App, HttpServer};
use receive_api_call::greet;

#[actix_web::main] // or #[tokio::main]
pub async fn router() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet)
            .service(calculate_decay)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
