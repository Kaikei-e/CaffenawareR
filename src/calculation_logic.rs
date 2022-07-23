mod sort_date;
mod string_to_date;

use crate::api_handler::api_structure::{FormValue, StartEndDate};
use crate::calculation_logic::sort_date::sort_date;

pub fn calc_tmax(mut form_value: FormValue) {
    let dates: StartEndDate = sort_date(form_value.date1, form_value.date2);

    form_value.start_end_date = dates;

    println!("{}", form_value.caffeine_mg);
}
