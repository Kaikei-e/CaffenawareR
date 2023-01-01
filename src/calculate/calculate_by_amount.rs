use crate::caffine_info::caffeine_info::{CaffeineInfo, CaffeineResult};

const TMAX_RATE: f64 = 1.1333;
const HALF_LIFE: f64 = 0.981924;

pub fn calculate_by_amount(caffeine: CaffeineInfo) -> CaffeineResult {
    let total_caffeine = total_caffeine_per100(caffeine.bottle_ml, caffeine.caffeine_mg);
    let caffeine_list: CaffeineResult = CaffeineResult {
        caffeine_mg: vec![],
        time: vec![],
    };

    let calculated_tmax = calculate_tmax(caffeine_list, total_caffeine);
    calculate_decay(calculated_tmax)
}

fn total_caffeine_per100(drink_amount: f64, caffeine_mg: f64) -> f64 {
    drink_amount * caffeine_mg / 100.0
}

fn calculate_tmax(caffe_list: CaffeineResult, total_caffeine: f64) -> CaffeineResult {
    let mut gain: f64 = 1.0;
    let mut caffeine_list = caffe_list;

    while total_caffeine > gain {
        gain *= TMAX_RATE;

        caffeine_list.caffeine_mg.push(gain);
        caffeine_list
            .time
            .push(caffeine_list.time.last().unwrap() + 1);
    }

    caffeine_list
}

// 2nd parameter:  , total_caffeine: f64
fn calculate_decay(mut results: CaffeineResult) -> CaffeineResult {
    let ref_results = results;

    let caffeine = ref_results.caffeine_mg.last();
    let caffeine_tmax = ref_results.caffeine_mg.last().unwrap().abs();
    let time = ref_results.time.last();

    let mut decay: Vec<f64> = Vec::new();
    let mut t: Vec<i64> = Vec::new();

    while caffeine_tmax > 5.0 {

        let caffeine = &caffeine.unwrap().abs();
        let time = &time.unwrap().abs();

        results.caffeine_mg.push(caffeine * HALF_LIFE);


        results.time.push(time + 60);
    }

    let results = CaffeineResult {
        caffeine_mg: decay,
        time: t,
    };

    results
}
