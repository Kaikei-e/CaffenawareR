mod sort_date;
mod string_to_date;
mod calc_struct;

use crate::api_handler::api_structure::{FormValue, StartEndDate};
use crate::calculation_logic::sort_date::sort_date;
use chrono::{Date, ParseError, Utc};
use crate::calculation_logic::calc_struct::decay_transition;

pub fn calc_tmax(mut form_value: FormValue) -> Result<StartEndDate, ParseError> {
    const calc_method2: i32 = 2;
    const drink_per_100ml: f64 = 100.0;
    const tmax: f64 = 1.1333;

    let dates: Result<StartEndDate, ParseError> = sort_date(form_value.date1, form_value.date2);

    form_value.start_end_date = dates.unwrap();

    let mut took_caffeine: i32;

    if form_value.calculate_method == calc_method2 {
        took_caffeine = form_value.caffeine_mg * form_value.drink_amount / drink_per_100ml
    } else {
        took_caffeine = form_value.caffeine_mg as i32
    }

    let mut to_max: f64;
    let mut date_at: Date<Utc>;

    let vec_decay: Vec<decay_transition> = Vec::new();



    dates
}
