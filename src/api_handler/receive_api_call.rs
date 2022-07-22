use crate::calculation_logic::calc_tmax;
use actix_web::cookie::time::error::Format;
use actix_web::{post, web, Error, HttpResponse, Responder};
use json::JsonValue;
use std::ptr::null;

#[post("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/api/calculate_caffeine")]
pub async fn calc_decay(body: &str) -> Result<HttpResponse, Error> {
    println!("received!!");

    let form_value: FormValue = serde_json::from_str(body)?;
    /*let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };*/
    calc_tmax(form_value.date1, form_value.date2);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}
