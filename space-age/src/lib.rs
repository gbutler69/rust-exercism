const SECONDS_PER_EARTH_YEAR: f64 = 31_557_600_f64;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($planet:ident orbital period $period:expr) => {
        pub struct $planet();
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / SECONDS_PER_EARTH_YEAR / $period
            }
        }
    };
}

planet! {Mercury orbital period 0.2408467}
planet! {Venus orbital period 0.61519726}
planet! {Earth orbital period 1.0}
planet! {Mars orbital period 1.8808158}
planet! {Jupiter orbital period 11.862615}
planet! {Saturn orbital period 29.447498}
planet! {Uranus orbital period 84.016846}
planet! {Neptune orbital period 164.79132}
