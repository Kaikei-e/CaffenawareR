use serde::{Deserialize, Serialize};
use std::f64;

#[derive(Deserialize, Serialize)]
pub struct CaffeineInfo {
    pub(crate) caffeine_mg: f64,
    pub(crate) time: i64,
    pub(crate) method: u8,
    pub(crate) bottle_ml: f64,
}

pub struct CaffeineResult {
    pub caffeine_mg: Vec<f64>,
    pub time: Vec<i64>,
}
