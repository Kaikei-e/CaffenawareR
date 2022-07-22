mod receive_api_call;

use actix_web::{App, HttpServer};
use receive_api_call::greet;

#[actix_web::main] // or #[tokio::main]
pub async fn router() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
