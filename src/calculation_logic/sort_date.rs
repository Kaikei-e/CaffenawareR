use crate::api_handler::api_structure::StartEndDate;
use std::os::unix::raw::time_t;

pub fn sort_date(date1: time_t, date2: time_t) -> StartEndDate {
    let mut items = vec![date1, date2];
    items.sort();

    let start_end_date = StartEndDate {
        start_date: items[0],
        end_date: items[1],
    };

    start_end_date
}
