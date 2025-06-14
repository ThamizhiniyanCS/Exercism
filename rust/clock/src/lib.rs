use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Convert hours to minutes and add minutes for easy calculation
        let total_minutes = hours * 60 + minutes;
        Self::generate_clock_from_total_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Convert hours to minutes and add minutes + additional for easy calculation
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        Self::generate_clock_from_total_minutes(total_minutes)
    }

    fn generate_clock_from_total_minutes(total_minutes: i32) -> Self {
        // We are using 24 * 60 (total minutes in a day) to make sure that we always get a valid
        // time within a 24-hour clock. This handles all the edge cases even if the input is
        // negative
        let total_minutes = total_minutes.rem_euclid(24 * 60);

        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
