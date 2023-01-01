use crate::caffine_info::caffeine_info;
use axum::response::IntoResponse;
use axum::Json;

pub(crate) async fn calculate_decay(
    Json(caffeine): Json<caffeine_info::CaffeineInfo>,
) -> impl IntoResponse {
    let calculated_result = calculate_logic(caffeine);

    Json(calculated_result)
}

pub fn calculate_logic(caffeine_info: caffeine_info::CaffeineInfo) -> caffeine_info::CaffeineInfo {
    caffeine_info
}
