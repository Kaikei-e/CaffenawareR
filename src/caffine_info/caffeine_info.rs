use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CaffeineInfo {
    pub(crate) caffeine_mg: f64,
    pub(crate) time: f64,
    pub(crate) method: u8,
    pub(crate) bottle_ml: f64,
}
