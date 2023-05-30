// Here we import std::fmt, so that we can
// implement Display trait for the struct Clock
use std::fmt;

// This approach only stores minutes in the
// clock struct. Any new clock is stored
//
const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock {
            minutes: (((hours * HOUR + minutes).rem_euclid(DAY)) + DAY).rem_euclid(DAY),
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes.div_euclid(HOUR),
            self.minutes.rem_euclid(HOUR)
        )
    }
}
