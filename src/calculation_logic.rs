mod sort_date;
mod string_to_date;

use crate::api_handler::api_structure::{FormValue, StartEndDate};
use crate::calculation_logic::sort_date::sort_date;
use chrono::ParseError;

pub fn calc_tmax(mut form_value: FormValue) -> Result<StartEndDate, ParseError> {
    let dates: Result<StartEndDate, ParseError> = sort_date(form_value.date1, form_value.date2);

    form_value.start_end_date = dates.unwrap();

    dates

}
