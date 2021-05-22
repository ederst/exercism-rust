use chrono::{Duration, NaiveTime};
use std::{cmp, fmt};

const CLOCK_FORMAT: &str = "%H:%M";

pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time: NaiveTime = NaiveTime::from_hms(0, 0, 0)
            + Duration::hours(hours as i64)
            + Duration::minutes(minutes as i64);

        Clock { time }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time: NaiveTime = self.time + Duration::minutes(minutes as i64);

        Clock { time }
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.format(CLOCK_FORMAT))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.format(CLOCK_FORMAT))
    }
}
