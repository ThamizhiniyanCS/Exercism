use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Avoid overflow of hours and minutes by converting out of range numbers back into the
        // valid range ( i.e., 0 <= hours <= 24 and 0 <= minutes <= 60 )
        // https://doc.rust-lang.org/stable/std/primitive.i32.html#method.wrapping_rem_euclid
        let mut h = hours.wrapping_rem_euclid(24);
        let m = minutes.wrapping_rem_euclid(60);

        // Checking if minutes is negative and if 0 < m < 60, then decrement h
        if minutes < 0 && m > 0 && m < 60 {
            h -= 1;
        }

        // Decrementing hours for every extra hour that we got from converting the excess minutes
        for _ in 0..(minutes / 60).abs() {
            match minutes > 0 {
                true => h += 1,
                false => h -= 1,
            }
        }

        // Wrapping the hours in case of overflow
        h = h.wrapping_rem_euclid(24);

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut h = self.hours;
        let m = (self.minutes + minutes).wrapping_rem_euclid(60);

        // Checking if minutes is negative and if 0 < m < 60, then decrement h
        if minutes < 0 && m > 0 && m < 60 {
            h -= 1;
        }

        // Decrementing hours for every extra hour that we got from converting the excess minutes
        for _ in 0..((self.minutes + minutes) / 60).abs() {
            match minutes > 0 {
                true => h += 1,
                false => h -= 1,
            }
        }

        // Wrapping the hours in case of overflow
        h = h.wrapping_rem_euclid(24);

        Clock {
            hours: h,
            minutes: m,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.to_string() == other.to_string()
//     }
// }
