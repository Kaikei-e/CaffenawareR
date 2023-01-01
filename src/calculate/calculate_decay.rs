use crate::caffine_info::caffeine_info;
use crate::caffine_info::caffeine_info::CaffeineResult;
use crate::calculate::calculate_by_amount::calculate_by_amount;
use axum::Json;

pub(crate) async fn calculate_decay(
    Json(caffeine): Json<caffeine_info::CaffeineInfo>,
) -> CaffeineResult {
    // let calculated_result = calculate_logic(caffeine)
    calculate_logic(caffeine)
}

pub fn calculate_logic(
    caffeine_info: caffeine_info::CaffeineInfo,
) -> CaffeineResult {
    const BY_CAFFEINE_AMOUNT: u8 = 1;
    const BY_DRINK_AMOUNT: u8 = 2;

    match caffeine_info.method {
        BY_CAFFEINE_AMOUNT => calculate_by_amount(caffeine_info),
        BY_DRINK_AMOUNT => calculate_by_amount(caffeine_info),
        _ => panic!(),
    }
}
