use crate::api_handler::api_structure::StartEndDate;
use crate::calculation_logic::string_to_date::parse_and_transform_date;
use std::time;

pub fn sort_date(date1: String, date2: String) -> StartEndDate {
    let parsed_d1 = parse_and_transform_date(date1);
    let parsed_d2 = parse_and_transform_date(date2);

    let d1 = parsed_d1.ok().unwrap();
    let d2 = parsed_d2.ok().unwrap();

    let mut items = vec![d1, d2];

    items.sort();

    let start_end_date = StartEndDate {
        start_date: items[0],
        end_date: items[1],
    };

    start_end_date
}
