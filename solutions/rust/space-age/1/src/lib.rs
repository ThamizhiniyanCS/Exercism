#[macro_export]
macro_rules! round_off_two_decimals {
    ($val:expr) => {
        ($val * 100.0).round() / 100.0
    };
}

#[macro_export]
macro_rules! earth_age_calc {
    ($val:expr) => {
        $val as f64 / 31_557_600 as f64
    };
}

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 0.2408467)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 0.61519726)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 1.0)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 1.8808158)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        round_off_two_decimals!(earth_age_calc!(d.seconds) / 164.79132)
    }
}
