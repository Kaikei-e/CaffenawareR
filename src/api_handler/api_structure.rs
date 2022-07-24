use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormValue {
    pub date1: String,
    pub date2: String,
    pub caffeine_mg: u16,
    pub drink_amount: u16,
    pub calculate_method: u8,
    pub start_end_date: Option<StartEndDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct StartEndDate {
    pub start_date: i64,
    pub end_date: i64,
}
