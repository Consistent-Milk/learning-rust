pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}
pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        let earth_year: f64 = 31557600 as f64;
        return d.seconds / (Self::period() * earth_year);
    }
}

macro_rules! planet {
    ($planet_name: ident, $planet_period: expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn period() -> f64 {
                return $planet_period;
            }
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
