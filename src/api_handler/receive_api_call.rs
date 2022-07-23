use crate::api_handler::api_structure::FormValue;
use crate::calculation_logic::calc_tmax;
use actix_web::cookie::time::error::Format;
use actix_web::{post, web, Error, HttpResponse, Responder};
use json::JsonValue;

#[post("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/api/calculate-caffeine")]
pub async fn calc_decay(body: String) -> impl Responder {
    println!("received!!");

    let form_value: FormValue = serde_json::from_str(&body).unwrap();

    let result = calc_tmax(form_value);

    match result {
        Ok(dates) => HttpResponse::Ok().body("received!!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
