use chrono::{format, NaiveDate, TimeZone, Utc};

pub fn parse_and_transform_date(date_str: String) -> Result<i64, format::ParseError> {
    let date = NaiveDate::parse_from_str(&*date_str, "%Y-%m-%d")?;
    let date_parsed: i64 = Utc
        .from_utc_date(&date)
        .and_hms_milli(0, 0, 1, 0)
        .timestamp_millis()
        .clamp(
            Utc.ymd(2016, 1, 1)
                .and_hms_milli(0, 0, 0, 0)
                .timestamp_millis(),
            Utc::now().timestamp_millis(),
        );
    Ok(date_parsed)
}
