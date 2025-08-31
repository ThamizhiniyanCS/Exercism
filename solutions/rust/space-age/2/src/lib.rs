static EARTH_SECONDS: f64 = 31_557_600.0;

macro_rules! planet {
    // https://doc.rust-lang.org/stable/rust-by-example/macros/designators.html
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn orbital_period() -> f64 {
                $orbital_period
            }
        }
    };
}

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / EARTH_SECONDS)
    }
}

pub trait Planet {
    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::orbital_period()
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
