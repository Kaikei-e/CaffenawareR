use serde::{Deserialize, Serialize};
use std::os::unix::raw::time_t;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormValue {
    pub date1: time_t,
    pub date2: time_t,
    pub caffeine_mg: u16,
    pub drink_amount: u16,
    pub calculate_method: u8,
    pub start_end_date: StartEndDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartEndDate {
    pub start_date: time_t,
    pub end_date: time_t,
}
