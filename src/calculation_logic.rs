mod calc_struct;
mod sort_date;
mod string_to_date;

use crate::api_handler::api_structure::{FormValue, StartEndDate};
use crate::calculation_logic::calc_struct::DecayTransition;
use crate::calculation_logic::sort_date::sort_date;
use chrono::{DateTime, Utc, Duration as ChronDur};
use serde_json::Error;
use std::borrow::Borrow;
use std::option::Option;
use std::time::{Duration, UNIX_EPOCH};

pub fn calc_tmax(mut form_value: FormValue) -> String {
    const CALC_METHOD2: u8 = 2;
    const DRINK_PER_100ML: f64 = 100.0;
    const TMAX: f64 = 1.1333;
    const DECAY_RATE: f64 = 0.99807;

    let dates: Result<StartEndDate, Error> = sort_date(form_value.date1, form_value.date2);
    let date_c = &dates.as_ref().unwrap();

    form_value.start_end_date = *Option::from(dates.unwrap()).borrow();

    let took_caffeine: i32;

    if form_value.calculate_method == CALC_METHOD2 {
        took_caffeine =
            ((form_value.caffeine_mg * form_value.drink_amount) as f64 / DRINK_PER_100ML) as i32
    } else {
        took_caffeine = form_value.caffeine_mg as i32
    }

    let mut to_max: f64 = 1.0;
    let dates_unwrap: StartEndDate = form_value.start_end_date.unwrap();
    let mut date_at: i64 = dates_unwrap.start_date;

    let mut vec_decay: Vec<DecayTransition> = Vec::new();

    let mut count = 0;
    while to_max < 10.0 * took_caffeine as f64 {
        let mut a_decay: DecayTransition = DecayTransition {
            time_line: 0,
            rest_caffeine: 1.0,
        };

        if count == 0 {
            a_decay.rest_caffeine = to_max;
            a_decay.time_line = date_at;

            vec_decay.push(a_decay);

            count += 1;
            continue;
        }

        count += 1;

        to_max *= TMAX;
        let add_minute: ChronDur = ChronDur::minutes(1);
        date_at += add_minute.num_minutes();

        if to_max > took_caffeine as f64 {
            a_decay.rest_caffeine = took_caffeine as f64;
            a_decay.time_line = date_at;

            break;
        }

        a_decay.rest_caffeine = took_caffeine as f64;
        a_decay.time_line = date_at;

        vec_decay.push(a_decay);
    }

    let mut to_zero: f64 = took_caffeine as f64;
    const CUT: f64 = 5.000;

    let vec_len = vec_decay.len();

    date_at = vec_decay[vec_len - 1].time_line;

    while to_zero > CUT {
        let mut a_decay: DecayTransition = DecayTransition {
            time_line: 0,
            rest_caffeine: 1.0,
        };

        let add_minute: ChronDur = ChronDur::minutes(1);
        date_at += add_minute.num_minutes();

        a_decay.time_line = date_at;
        to_zero *= DECAY_RATE;
        a_decay.rest_caffeine = to_zero;

        vec_decay.push(a_decay);
    }

    println!("{}", vec_decay[0].time_line);

    let d_unix = UNIX_EPOCH + Duration::from_secs(date_c.start_date as u64);
    // Create DateTime from SystemTime
    let return_datetime = DateTime::<Utc>::from(d_unix);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = return_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    timestamp_str
}
