use crate::api_handler::api_structure::StartEndDate;
use crate::calculation_logic::string_to_date::parse_and_transform_date;
use chrono::ParseError;

pub fn sort_date(date1: String, date2: String) -> Result<StartEndDate, ParseError> {
    let parsed_d1 = parse_and_transform_date(date1);
    let parsed_d2 = parse_and_transform_date(date2);

    let d1 = parsed_d1.unwrap();
    let d2 = parsed_d2.unwrap();

    let mut items = vec![d1, d2];

    items.sort();

    Ok(StartEndDate {
        start_date: items[0],
        end_date: items[1],
    })
}
