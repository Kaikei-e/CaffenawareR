use crate::api_handler::api_structure::FormValue;
use crate::calculation_logic::calc_tmax;
use actix_web::cookie::time::error::Format;
use actix_web::{post, web, Error, HttpResponse, Responder};
use json::JsonValue;

#[post("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/api/calculate_caffeine")]
pub async fn calc_decay(body: String) -> HttpResponse {
    println!("received!!");

    let form_value: FormValue = serde_json::from_str(&body)?;

    let result = calc_tmax(form_value);

    //let result = HttpResponse
    //    .content_type("application/json")
    //    .body(serde_json::to_string(&form_value));

    match result {
        Ok(dates) => HttpResponse::Ok().json(serde_json::to_string(&dates)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
