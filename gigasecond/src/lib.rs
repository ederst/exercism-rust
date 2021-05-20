use chrono::{DateTime, Utc, Duration};

//const GIGASECOND: i64 = 1_000_000_000;
const GIGASECOND: i64 = i64::pow(10, 9);

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(GIGASECOND)
}
