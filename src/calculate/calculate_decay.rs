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
    const BY_CAFFEINE_AMOUNT: u8 = 1;
    const BY_DRINK_AMOUNT: u8 = 2;

    match caffeine_info.method {
        BY_CAFFEINE_AMOUNT => caffeine_info,
        BY_DRINK_AMOUNT => caffeine_info,
        _ => panic!(),
    }
}
