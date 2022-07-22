use std::os::unix::raw::time_t;

pub fn sort_date(date1: time_t, date2: time_t) {
    let mut items = vec![date1, date2];
    items.sort();
}
