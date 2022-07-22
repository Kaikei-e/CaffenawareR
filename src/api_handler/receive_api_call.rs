use actix_web::{post, web, HttpResponse, Responder, Error};
use std::os::unix::raw::time_t;
use std::ptr::null;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[post("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Debug, Serialize, Deserialize)]
struct FormValue {
    start_date: time_t,
    end_date: time_t,
    caffeine_mg: u16,
    drink_amount: u16,
    calculate_method: u8,
}

#[post("/api/calculate_caffeine")]
pub async fn calculate_decay(body: web::Bytes) -> Result<HttpResponse, Error> {
    println!("received!!");

    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}
