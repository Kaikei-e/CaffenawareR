mod calc_struct;
mod sort_date;
mod string_to_date;

use crate::api_handler::api_structure::{FormValue, StartEndDate};
use crate::calculation_logic::calc_struct::decay_transition;
use crate::calculation_logic::sort_date::sort_date;
use chrono::{Date, Duration, ParseError, Utc};
use std::ops::Add;

pub fn calc_tmax(mut form_value: FormValue) -> Result<StartEndDate, ParseError> {
    const CALC_METHOD2: i32 = 2;
    const DRINK_PER_100ML: f64 = 100.0;
    const TMAX: f64 = 1.1333;
    const DECAY_RATE: f64 = 0.99807;

    let dates: Result<StartEndDate, ParseError> = sort_date(form_value.date1, form_value.date2);

    form_value.start_end_date = dates.unwrap();

    let mut took_caffeine: i32;

    if form_value.calculate_method == CALC_METHOD2 {
        took_caffeine = form_value.caffeine_mg * form_value.drink_amount / DRINK_PER_100ML
    } else {
        took_caffeine = form_value.caffeine_mg as i32
    }

    let mut to_max: f64 = 1.0;
    let mut date_at: i64 = form_value.start_end_date.start_date;

    let mut vec_decay: Vec<decay_transition> = Vec::new();

    for i in 0..to_max < 10.0 * took_caffeine {
        let mut a_decay: decay_transition = decay_transition {
            time_line: 0,
            rest_caffeine: 1.0,
        };

        if i == 0 {
            a_decay.rest_caffeine = to_max;
            a_decay.time_line = date_at;

            vec_decay.push(a_decay);
            continue;
        }

        to_max *= TMAX;
        const ADD_MINUTE: Duration = Duration::minutes(1);
        date_at += ADD_MINUTE;

        if to_max > took_caffeine as f64 {
            a_decay.rest_caffeine = took_caffeine as f64;
            a_decay.time_line = date_at;

            break;
        }

        a_decay.rest_caffeine = took_caffeine as f64;
        a_decay.time_line = date_at;

        vec_decay.push(a_decay);
    }

    const TO_ZERO: f64 = 1.0;

    TO_ZERO = took_caffeine as f64;
    let vec_len = vec_decay.len();

    date_at = vec_decay[vec_len - 1].time_line;

    dates
}
