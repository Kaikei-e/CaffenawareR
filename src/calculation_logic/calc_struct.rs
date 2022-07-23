use chrono::{Date, Utc};

pub struct decay_transition {
    time_line: Date<Utc>,
    rest_caffeine: f64,
}
