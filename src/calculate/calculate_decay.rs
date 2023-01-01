use crate::caffeine_info::caffeine_info;
use crate::caffeine_info::caffeine_info::CaffeineInfo;
use crate::calculate::calculate_by_amount::calculate_by_amount;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn calculate_decay(Json(caffeine): Json<CaffeineInfo>) -> impl IntoResponse {
    // let calculated_result = calculate_logic(caffeine)
    let fully_calculated = calculate_logic(caffeine);

    (StatusCode::OK, Json(fully_calculated))
}

pub fn calculate_logic(caffeine_info: CaffeineInfo) -> caffeine_info::CaffeineResult {
    const BY_CAFFEINE_AMOUNT: u8 = 1;
    const BY_DRINK_AMOUNT: u8 = 2;

    match caffeine_info.method {
        BY_CAFFEINE_AMOUNT => calculate_by_amount(caffeine_info),
        BY_DRINK_AMOUNT => calculate_by_amount(caffeine_info),
        _ => panic!(),
    }
}
